use rustc_type_ir::inherent::*;
use rustc_type_ir::{self as ty, Interner};
use tracing::instrument;

use crate::delegate::SolverDelegate;
use crate::solve::{
    CandidateSource, Certainty, EvalCtxt, Goal, GoalSource, NoSolution, QueryResult,
};

impl<D, I> EvalCtxt<'_, D>
where
    D: SolverDelegate<Interner = I>,
    I: Interner,
{
    /// Resolve an associated trait bound, e.g. `B: <C as Container>::Elem`.
    ///
    /// For concrete self types in the projection's trait ref, this assembles
    /// impl candidates to extract the value trait bounds from the impl's
    /// associated trait item. For abstract self types (params, aliases), only
    /// the declaration bounds from the trait definition are enforced.
    #[instrument(level = "trace", skip(self), ret)]
    pub(super) fn compute_assoc_trait_bound_goal(
        &mut self,
        goal: Goal<I, ty::AssocTraitBoundPredicate<I>>,
    ) -> QueryResult<I> {
        let cx = self.cx();
        let pred = goal.predicate;
        let goal_trait_ref = pred.projection.trait_ref(cx);
        let trait_assoc_def_id = pred.projection.def_id;

        // Structurally normalize the self type of the projection's trait ref.
        let self_ty = goal_trait_ref.self_ty();
        let Ok(self_ty) = self.structurally_normalize_ty(goal.param_env, self_ty) else {
            return self.evaluate_added_goals_and_make_canonical_response(Certainty::AMBIGUOUS);
        };

        // If self type is still an inference variable, stall.
        if self_ty.is_ty_var() {
            return self.evaluate_added_goals_and_make_canonical_response(Certainty::AMBIGUOUS);
        }

        // Reconstruct the trait ref with the normalized self type.
        let goal_trait_ref = goal_trait_ref.with_replaced_self_ty(cx, self_ty);

        // For abstract self types (params, aliases, placeholders), we cannot
        // determine the specific impl. Only add the parent trait obligation.
        // Declaration bounds and impl-specific value bounds are enforced through
        // compare_impl_assoc_trait when the concrete impl is known.
        match self_ty.kind() {
            ty::Param(_) | ty::Alias(_, _) | ty::Placeholder(..) | ty::Error(_) => {
                self.add_goal(
                    GoalSource::Misc,
                    goal.with(
                        cx,
                        ty::TraitPredicate {
                            trait_ref: goal_trait_ref,
                            polarity: ty::PredicatePolarity::Positive,
                        },
                    ),
                );

                return self.evaluate_added_goals_and_make_canonical_response(Certainty::Yes);
            }
            _ => {}
        }

        // For concrete self types: assemble impl candidates, extract value bounds.
        let mut candidates = Vec::new();
        cx.for_each_relevant_impl(goal_trait_ref.def_id, self_ty, |impl_def_id| {
            if cx.impl_is_default(impl_def_id) {
                return;
            }

            let result =
                self.probe_trait_candidate(CandidateSource::Impl(impl_def_id)).enter(|ecx| {
                    let impl_args = ecx.fresh_args_for_item(impl_def_id.into());
                    let impl_trait_ref = cx.impl_trait_ref(impl_def_id).instantiate(cx, impl_args);

                    ecx.eq(goal.param_env, goal_trait_ref, impl_trait_ref)?;

                    // Add impl where-clause bounds.
                    let where_clause_bounds = cx
                        .predicates_of(impl_def_id.into())
                        .iter_instantiated(cx, impl_args)
                        .map(|pred| goal.with(cx, pred));
                    ecx.add_goals(GoalSource::ImplWhereBound, where_clause_bounds);

                    // Find the impl's associated trait item.
                    match ecx.fetch_eligible_assoc_item(
                        goal_trait_ref,
                        trait_assoc_def_id,
                        impl_def_id,
                    ) {
                        Ok(Some(impl_item_def_id)) => {
                            // Extract value trait bounds from the impl item.
                            for bound in cx.item_bounds(impl_item_def_id).skip_binder() {
                                if let Some(trait_clause) = bound.as_trait_clause() {
                                    let value_trait_ref = trait_clause.skip_binder().trait_ref;
                                    let new_trait_ref = ty::TraitRef::new(
                                        cx,
                                        value_trait_ref.def_id,
                                        [pred.self_ty],
                                    );
                                    ecx.add_goal(
                                        GoalSource::Misc,
                                        goal.with(
                                            cx,
                                            ty::TraitPredicate {
                                                trait_ref: new_trait_ref,
                                                polarity: ty::PredicatePolarity::Positive,
                                            },
                                        ),
                                    );
                                }
                            }
                        }
                        Ok(None) => {}
                        Err(_guar) => {}
                    }

                    ecx.evaluate_added_goals_and_make_canonical_response(Certainty::Yes)
                });

            match result {
                Ok(candidate) => candidates.push(candidate),
                Err(NoSolution) => {}
            }
        });

        if let Some((response, _)) = self.try_merge_candidates(&candidates) {
            Ok(response)
        } else {
            self.flounder(&candidates)
        }
    }
}

//@ check-pass
// Jezza's Component/UI pattern from rust-lang/rfcs#2190.
// Traits for UI framework with associated event constraints.

#![feature(associated_traits)]

trait Component {
    type Props: Clone + 'static;
    trait Events;

    fn new(props: Self::Props) -> Self;
}

trait ClickEvent {}
trait KeyEvent {}

struct OnClick;
impl ClickEvent for OnClick {}

struct OnKeyPress;
impl KeyEvent for OnKeyPress {}

struct Label {
    value: String,
}

struct LabelProps {
    value: String,
}
impl Clone for LabelProps {
    fn clone(&self) -> Self {
        LabelProps { value: self.value.clone() }
    }
}

impl Component for Label {
    type Props = LabelProps;
    trait Events = ClickEvent;

    fn new(props: Self::Props) -> Self {
        Label { value: props.value }
    }
}

// Generic context that uses both associated type and trait
fn handle_event<C: Component, E: C::Events>(_component: &C, _event: E) {}

// Using impl Trait with associated trait
fn handle_event_impl<C: Component>(_component: &C, _event: impl C::Events) {}

fn main() {
    let label = Label::new(LabelProps { value: "hello".to_string() });
    handle_event(&label, OnClick);
    handle_event_impl(&label, OnClick);
}

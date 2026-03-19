// Real-world example: a plugin system where plugins declare their
// required capabilities via associated traits.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Plugin {
    trait Capability;

    fn name(&self) -> &str;
}

struct AudioPlugin;
impl Plugin for AudioPlugin {
    trait Capability = Send + Sync; // audio plugins must be thread-safe
    fn name(&self) -> &str { "audio" }
}

struct UIPlugin;
impl Plugin for UIPlugin {
    trait Capability = Send; // UI plugins only need Send
    fn name(&self) -> &str { "ui" }
}

// Plugin host that requires capabilities
fn load_plugin<P: Plugin, C: P::Capability>(plugin: P, capability: C) -> String {
    format!("loaded: {}", plugin.name())
}

// Multiple associated traits in one plugin system
trait AdvancedPlugin {
    trait InputConstraint;
    trait OutputConstraint;

    fn process(&self);
}

struct TransformPlugin;
impl AdvancedPlugin for TransformPlugin {
    trait InputConstraint = Clone;
    trait OutputConstraint = Send + Clone;

    fn process(&self) {}
}

fn pipeline<P: AdvancedPlugin, I: P::InputConstraint, O: P::OutputConstraint>(
    plugin: P,
    _input: I,
    _output: O,
) {
    plugin.process();
}

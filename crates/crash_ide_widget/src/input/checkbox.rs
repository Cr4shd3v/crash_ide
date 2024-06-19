use bevy::prelude::*;

pub(super) struct CheckboxPlugin;

impl Plugin for CheckboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, create_checkbox)
        ;
    }
}

/// This bundle represents a checkbox with optional label next to it
#[derive(Bundle, Default)]
pub struct CheckboxBundle {
    /// [NodeBundle] for this element.
    ///
    /// This refers to the container around the checkbox + label
    pub node_bundle: NodeBundle,
    /// Value of the checkbox
    pub checkbox: Checkbox,
    /// Label of the checkbox
    pub label: CheckboxLabel,
}

/// Represents the value of a checkbox.
#[derive(Component, Default)]
pub enum Checkbox {
    /// Checkbox is checked
    Checked,
    /// Checkbox is not checked
    #[default]
    Unchecked,
}

/// Contains the label definition.
#[derive(Component, Default)]
pub enum CheckboxLabel {
    /// No label will be rendered
    #[default]
    None,
    /// Label will be created right to the checkbox
    Right(TextSection),
    /// Label will be created left to the checkbox
    Left(TextSection),
}

fn create_checkbox(
    query: Query<(Entity, &CheckboxLabel), Added<Checkbox>>,
) {
    for (_entity, _label) in query.iter() {

    }
}

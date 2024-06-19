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
    /// Styling of the checkbox
    pub checkbox_style: CheckboxStyle,
    /// Interaction to detect the click
    pub interaction: Interaction,
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

impl Checkbox {
    /// Whether self is [Checkbox::Checked]
    pub fn is_checked(&self) -> bool {
        matches!(self, Checkbox::Checked)
    }

    /// Creates a [Checkbox] from a bool
    pub fn from_bool(value: bool) -> Self {
        if value {
            Checkbox::Checked
        } else {
            Checkbox::Unchecked
        }
    }
}

/// Contains the label definition.
#[derive(Component, Default)]
pub enum CheckboxLabel {
    /// No label will be rendered
    #[default]
    None,
    /// Label will be created right to the checkbox
    End(TextSection),
    /// Label will be created left to the checkbox
    Start(TextSection),
}

/// Styling for a checkbox
#[derive(Component)]
pub struct CheckboxStyle {
    /// Color when the checkbox is checked
    pub active_color: Color,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        Self {
            active_color: Color::GRAY.with_a(0.1),
        }
    }
}

fn create_checkbox(
    mut commands: Commands,
    query: Query<(Entity, &CheckboxLabel, &Checkbox, &CheckboxStyle), Added<Checkbox>>,
) {
    for (entity, label, value, checkbox_style) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            if let CheckboxLabel::Start(section) = label {
                parent.spawn(TextBundle::from_sections(vec![section.clone()]));
            }

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(12.0),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                border_color: BorderColor(Color::GRAY.with_a(0.1)),
                background_color: BackgroundColor(if value.is_checked() { checkbox_style.active_color } else { Color::NONE }),
                ..default()
            });

            if let CheckboxLabel::End(section) = label {
                parent.spawn(TextBundle::from_sections(vec![section.clone()]));
            }
        });
    }
}

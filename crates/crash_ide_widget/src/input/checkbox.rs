use bevy::prelude::*;

pub(super) struct CheckboxPlugin;

impl Plugin for CheckboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (create_checkbox, toggle_checkbox))
        ;
    }
}

/// Global resource that can be inserted to set the default icon for all checkboxes.
#[derive(Resource)]
pub struct CheckboxDefaultIcon(pub Handle<Image>);

/// This bundle represents a checkbox with optional label next to it
#[derive(Bundle)]
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

impl Default for CheckboxBundle {
    fn default() -> Self {
        Self {
            node_bundle: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            interaction: Interaction::default(),
            checkbox: Checkbox::default(),
            label: CheckboxLabel::default(),
            checkbox_style: CheckboxStyle::default(),
        }
    }
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
#[derive(Component, Default)]
pub struct CheckboxStyle {
    /// Color when the checkbox is checked
    pub active_icon: Handle<Image>,
}

#[derive(Component)]
struct CheckboxDisplayRef(Entity);

fn create_checkbox(
    mut commands: Commands,
    query: Query<(Entity, &CheckboxLabel, &Checkbox, &CheckboxStyle), Added<Checkbox>>,
    default_checkbox: Option<Res<CheckboxDefaultIcon>>,
) {
    for (entity, label, value, checkbox_style) in query.iter() {
        let display_entity = commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(15.0),
                height: Val::Px(15.0),
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::horizontal(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::GRAY.with_a(0.1)),
            ..default()
        }).with_children(|parent| {
            if value.is_checked() {
                spawn_checkbox(&default_checkbox, checkbox_style, parent);
            }
        }).id();

        commands.entity(entity).insert(CheckboxDisplayRef(display_entity)).with_children(|parent| {
            if let CheckboxLabel::Start(section) = label {
                parent.spawn(TextBundle::from_sections(vec![section.clone()]));
            }
        }).add_child(display_entity).with_children(|parent| {
            if let CheckboxLabel::End(section) = label {
                parent.spawn(TextBundle::from_sections(vec![section.clone()]));
            }
        });
    }
}

fn toggle_checkbox(
    mut commands: Commands,
    mut query: Query<(&Interaction, &CheckboxDisplayRef, &mut Checkbox, &CheckboxStyle), Changed<Interaction>>,
    default_checkbox: Option<Res<CheckboxDefaultIcon>>,
) {
    for (interaction, display_ref, mut checkbox, checkbox_style) in query.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if checkbox.is_checked() {
            *checkbox = Checkbox::Unchecked;
            commands.entity(display_ref.0).despawn_descendants();
        } else {
            *checkbox = Checkbox::Checked;
            commands.entity(display_ref.0).with_children(|parent| {
                spawn_checkbox(&default_checkbox, checkbox_style, parent);
            });
        }
    }
}

fn spawn_checkbox(default_checkbox: &Option<Res<CheckboxDefaultIcon>>, checkbox_style: &CheckboxStyle, parent: &mut ChildBuilder) {
    parent.spawn(ImageBundle {
        image: UiImage::new(
            default_checkbox.as_ref().map(|v| v.0.clone())
                .unwrap_or(checkbox_style.active_icon.clone())
        ),
        style: Style {
            width: Val::Px(15.0),
            height: Val::Px(15.0),
            ..default()
        },
        ..default()
    });
}

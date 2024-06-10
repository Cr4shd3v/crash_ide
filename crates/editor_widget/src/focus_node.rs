use bevy::prelude::*;

pub(super) struct FocusNodePlugin;

impl Plugin for FocusNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, check_focus_node_despawn)
        ;
    }
}

/// An ui node with this component will despawn once a click happens outside of this node.
///
/// Requires [Interaction] for the entity.
#[derive(Component)]
pub struct FocusNode;

fn check_focus_node_despawn(
    mut commands: Commands,
    query: Query<(Entity, &Interaction), With<FocusNode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.any_just_pressed([MouseButton::Right, MouseButton::Middle, MouseButton::Left]) {
        for (entity, interaction) in query.iter() {
            if *interaction == Interaction::None {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
use bevy::prelude::*;
use crate::Hovered;

pub(super) struct RightClickedPlugin;

impl Plugin for RightClickedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, on_right_clicked)
        ;
    }
}

/// Marker struct when an ui element is right clicked
#[derive(Component)]
pub struct RightClicked;

fn on_right_clicked(
    mut commands: Commands,
    keys: Res<ButtonInput<MouseButton>>,
    hovered_query: Query<Entity, (With<Hovered>, With<Interaction>)>,
    right_clicked_query: Query<Entity, With<RightClicked>>,
) {
    if keys.just_pressed(MouseButton::Right) {
        for entity in hovered_query.iter() {
            commands.entity(entity).insert(RightClicked);
        }
    } else if keys.just_released(MouseButton::Right) {
        for entity in right_clicked_query.iter() {
            commands.entity(entity).remove::<RightClicked>();
        }
    }
}

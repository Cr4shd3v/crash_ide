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
pub struct RightClickable;

/// Trigger for when an element marked with [RightClickable] is right-clicked
#[derive(Event)]
pub struct RightClicked;

fn on_right_clicked(
    mut commands: Commands,
    keys: Res<ButtonInput<MouseButton>>,
    hovered_query: Query<Entity, (With<Hovered>, With<Interaction>, With<RightClickable>)>,
) {
    if keys.just_pressed(MouseButton::Right) {
        for entity in hovered_query.iter() {
            commands.trigger_targets(RightClicked, entity);
        }
    }
}

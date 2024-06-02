use std::time::Instant;
use bevy::prelude::*;

/// Marks an element to be double-clickable.
///
/// When double-clicked, the component will temporarily get a [DoubleClicked] component.
#[derive(Component, Default)]
pub struct DoubleClickButton {
    last_click: Option<Instant>,
}

/// Component that marks this node a double-clicked.
#[derive(Component)]
pub struct DoubleClicked;

pub(super) fn double_click_detection(
    mut commands: Commands,
    mut query: Query<(Entity, &Interaction, &mut DoubleClickButton), Changed<Interaction>>
) {
    for (entity, interaction, mut double_click_button) in query.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Some(last_click) = double_click_button.last_click {
            if Instant::now().duration_since(last_click).as_millis() <= 750 {
                commands.entity(entity).insert(DoubleClicked);
                double_click_button.last_click = None;
            } else {
                double_click_button.last_click = Some(Instant::now());
            }
        } else {
            double_click_button.last_click = Some(Instant::now());
        }
    }
}

pub(super) fn remove_double_click(
    mut commands: Commands,
    query: Query<Entity, With<DoubleClicked>>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<DoubleClicked>();
    }
}
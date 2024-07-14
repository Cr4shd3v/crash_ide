use bevy::prelude::*;

pub(super) struct TriggerPlugin;

impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, trigger_clicked)
        ;
    }
}

#[derive(Event)]
pub struct Clicked;

fn trigger_clicked(
    mut commands: Commands,
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    for (entity, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        commands.trigger_targets(Clicked, entity);
    }
}
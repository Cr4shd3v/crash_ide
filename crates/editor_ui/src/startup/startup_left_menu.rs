use bevy::prelude::*;

pub(crate) fn startup_left_menu(builder: &mut ChildBuilder) {
    builder.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(30.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::BLUE),
        ..default()
    }).with_children(|_parent| {

    });
}
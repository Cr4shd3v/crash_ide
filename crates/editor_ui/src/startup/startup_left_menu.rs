use bevy::prelude::*;

pub(crate) fn startup_left_menu(builder: &mut ChildBuilder) {
    builder.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(30.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        startup_left_menu_entry(parent, "Projects");
    });
}

#[derive(Component)]
struct StartupLeftMenuEntry {
    pub name: String,
}

fn startup_left_menu_entry(builder: &mut ChildBuilder, title: &str) {
    builder.spawn((NodeBundle {
        style: Style {
            padding: UiRect::axes(Val::Percent(5.0), Val::Percent(4.0)),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        ..default()
    }, StartupLeftMenuEntry {
        name: title.to_string(),
    })).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(title, TextStyle::default()),
            ..default()
        });
    });
}
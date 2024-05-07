use bevy::prelude::*;
use crate::startup::StartupScreenState;
use crate::widget::Hoverable;

pub(crate) fn startup_left_menu(builder: &mut ChildBuilder) {
    builder.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(20.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        startup_left_menu_entry(parent, StartupScreenState::ProjectSelect);
        startup_left_menu_entry(parent, StartupScreenState::Settings);
    });
}

#[derive(Component)]
pub(crate) struct StartupLeftMenuEntry {
    pub next_state: StartupScreenState,
}

impl StartupLeftMenuEntry {
    pub fn new(next_state: StartupScreenState) -> Self {
        Self {
            next_state,
        }
    }
}

fn startup_left_menu_entry(builder: &mut ChildBuilder, next_state: StartupScreenState) {
    builder.spawn((ButtonBundle {
        style: Style {
            padding: UiRect::axes(Val::Percent(5.0), Val::Percent(4.0)),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..default()
    }, StartupLeftMenuEntry::new(next_state), Hoverable::new(Color::GRAY.with_a(0.2)),
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(next_state.title(), TextStyle {
                font_size: 16.0,
                ..default()
            }),
            ..default()
        });
    });
}

pub(crate) fn startup_left_menu_click(
    state: Res<State<StartupScreenState>>,
    mut next_state: ResMut<NextState<StartupScreenState>>,
    click_query: Query<(&Interaction, &StartupLeftMenuEntry), Changed<Interaction>>,
) {
    for (interaction, menu_entry) in click_query.iter() {
        if matches!(interaction, Interaction::Pressed) && *state != menu_entry.next_state {
            next_state.set(menu_entry.next_state);
        }
    }
}
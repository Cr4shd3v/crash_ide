use bevy::prelude::*;
use crash_ide_widget::Hoverable;
use crash_ide_assets::DefaultFonts;
use crate::startup::StartupScreenState;

pub(crate) fn startup_left_menu(builder: &mut ChildBuilder) {
    builder.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(20.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        border_color: BorderColor(Color::GRAY.with_a(0.16)),
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
        parent.spawn(TextBundle::from_section(next_state.title(), TextStyle {
            font_size: 16.0,
            font: DefaultFonts::ROBOTO_REGULAR,
            ..default()
        }));
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

pub(crate) fn handle_left_menu_state_change(
    state: Res<State<StartupScreenState>>,
    mut color_query: Query<(&mut BackgroundColor, &StartupLeftMenuEntry)>,
) {
    if !state.is_changed() {
        return;
    }

    let active_title = state.title();

    for (mut color, menu_entry) in color_query.iter_mut() {
        if menu_entry.next_state.title() == active_title {
            color.0 = Color::GRAY.with_a(0.4);
        } else {
            color.0 = Color::NONE;
        }
    }
}
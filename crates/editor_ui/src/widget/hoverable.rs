use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(super) struct HoverablePlugin;

impl Plugin for HoverablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_hover, on_hover));
    }
}

#[derive(Component)]
pub struct Hoverable {
    hover_color: Color,
    saved_color: Option<Color>,
}

impl Hoverable {
    pub fn new(hover_color: Color) -> Self {
        Self {
            hover_color,
            saved_color: None,
        }
    }
}

fn init_hover(mut query: Query<(&mut Hoverable, &BackgroundColor), Added<Hoverable>>) {
    for (mut hoverable, background_color) in query.iter_mut() {
        hoverable.saved_color = Some(background_color.0);
    }
}

fn on_hover(
    mut interaction_query: Query<(&Interaction, &mut Hoverable, &mut BackgroundColor), Changed<Interaction>>,
    mut p_window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = p_window_query.single_mut();
    let mut is_hovered_this_run = false;

    for (interaction, mut hoverable, mut background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                hoverable.saved_color = Some(background_color.0);
                background_color.0 = hoverable.hover_color;
                primary_window.cursor.icon = CursorIcon::Pointer;
                is_hovered_this_run = true;
            }
            Interaction::None => {
                if let Some(ref color) = hoverable.saved_color {
                    background_color.0 = color.clone();
                    if !is_hovered_this_run {
                        primary_window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
            _ => {},
        }
    }
}
use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::widget::screen::CreateProjectWindow;

#[derive(Component, Default)]
pub struct CreateProjectButton {
    pub base_window: Option<Entity>,
}

pub(super) fn create_project_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &CreateProjectButton), Changed<Interaction>>,
    mut create_project_window_query: Query<&mut Window, With<CreateProjectWindow>>,
) {
    for (interaction, button) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if let Ok(mut window) = create_project_window_query.get_single_mut() {
                    window.focused = true;
                } else {
                    commands.spawn((
                        Window {
                            title: "Create Project".to_string(),
                            resolution: WindowResolution::new(1000.0, 700.0).with_scale_factor_override(1.0),
                            ..default()
                        },
                        CreateProjectWindow {
                            base_window: button.base_window,
                        },
                    ));
                }
            }
            _ => {}
        }
    }
}
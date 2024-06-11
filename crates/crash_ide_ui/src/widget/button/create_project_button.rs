use bevy::prelude::*;
use bevy::window::WindowResolution;
use crash_ide_widget::ActiveWindow;
use crate::widget::screen::CreateProjectWindow;

#[derive(Component)]
pub struct CreateProjectButton;

pub(super) fn create_project_button(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (With<CreateProjectButton>, Changed<Interaction>)>,
    mut create_project_window_query: Query<&mut Window, With<CreateProjectWindow>>,
    current_window_query: Query<Entity, With<ActiveWindow>>,
) {
    for interaction in interaction_query.iter() {
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
                            base_window: Some(current_window_query.single()),
                        },
                    ));
                }
            }
            _ => {}
        }
    }
}
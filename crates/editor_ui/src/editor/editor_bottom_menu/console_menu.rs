use std::io::Read;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use editor_assets::DefaultFonts;
use editor_config::FindProjectInParents;
use editor_widget::{TextInputBundle, TextInputSettings, TextInputTextStyle};
use crate::editor::main_editor_screen::EditorBottomMenu;

pub(super) struct ConsoleMenuPlugin;

impl Plugin for ConsoleMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_console_instance, spawn_console_progress, console_stdout))
        ;
    }
}

#[derive(Component)]
struct ConsoleInstance {
    directory: PathBuf,
}

fn spawn_console_instance(
    mut commands: Commands,
    query: Query<Entity, Added<EditorBottomMenu>>,
    find_project_in_parents: FindProjectInParents,
) {
    for entity in query.iter() {
        let project = find_project_in_parents.find(entity);

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, ConsoleInstance {
                directory: PathBuf::from(&project.editor_project.path),
            })).with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(format!("{}$", project.editor_project.path), TextStyle {
                            font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                            font_size: 18.0,
                            ..default()
                        }),
                        style: Style {
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    }, TextInputBundle {
                        text_input_text_style: TextInputTextStyle::default().with_font(DefaultFonts::JETBRAINS_MONO_REGULAR),
                        text_input_settings: TextInputSettings {
                            with_border: false,
                            ..default()
                        },
                        ..default()
                    }));
                });
            });
        });
    }
}

#[derive(Component)]
struct ConsoleCommand {
    command: String,
    directory: PathBuf,
}

#[derive(Component)]
struct ConsoleProgress {
    #[allow(unused)]
    cmd: Child,
    pub stdout_read: Arc<Mutex<Receiver<Vec<u8>>>>,
    stdout_task: Task<()>,
    stderr_task: Task<()>,
}

fn spawn_console_progress(
    mut commands: Commands,
    query: Query<(Entity, &ConsoleCommand), Added<ConsoleCommand>>,
) {
    for (entity, command) in query.iter() {
        #[cfg(target_os = "windows")]
        let cmd = Command::new("cmd")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        #[cfg(target_os = "linux")]
        let cmd = Command::new("bash")
            .arg("-c")
            .arg(&command.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&command.directory)
            .spawn();

        if let Err(e) = cmd {
            commands.entity(entity).despawn_descendants().with_children(|parent| {
                parent.spawn(TextBundle::from_section(format!("Could not start console: {}", e), TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    ..default()
                }));
            });

            continue;
        }

        let mut cmd = cmd.unwrap();

        let mut stdout = cmd.stdout.take().unwrap();
        let mut stderr = cmd.stderr.take().unwrap();

        let (stdout_write, stdout_read) = channel();

        let pool = AsyncComputeTaskPool::get();

        let cloned_write = stdout_write.clone();
        let stdout_task = pool.spawn(async move {
            let mut buf = [0u8; 512];
            while let Ok(_) = stdout.read(&mut buf) {
                if cloned_write.send(buf.to_vec()).is_err() {
                    break;
                }

                buf = [0u8; 512];
            }
        });

        let stderr_task = pool.spawn(async move {
            let mut buf = [0u8; 512];
            while let Ok(_) = stderr.read(&mut buf) {
                if stdout_write.send(buf.to_vec()).is_err() {
                    break;
                }

                buf = [0u8; 512];
            }
        });

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((ConsoleProgress {
                cmd,
                stdout_read: Arc::new(Mutex::new(stdout_read)),
                stdout_task,
                stderr_task,
            }, TextBundle::from_section(command.command.clone(), TextStyle {
                font: DefaultFonts::ROBOTO_REGULAR,
                ..default()
            })));
        });
    }
}

fn console_stdout(
    mut query: Query<(&mut Text, &ConsoleProgress)>
) {
    for (mut text, console) in query.iter_mut() {
        let mut result = vec![];
        while let Ok(content) = console.stdout_read.lock().unwrap().try_recv() {
            result.extend(content);
        }

        if !result.is_empty() {
            text.sections[0].value.push_str(&*String::from_utf8(result).unwrap());
        }
    }
}
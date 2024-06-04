use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use editor_assets::DefaultFonts;
use crate::editor::main_editor_screen::EditorBottomMenu;

pub(super) struct ConsoleMenuPlugin;

impl Plugin for ConsoleMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_console_menu, console_stdout))
        ;
    }
}

#[derive(Component)]
struct Console {
    #[allow(unused)]
    cmd: Child,
    pub stdin_write: Sender<Vec<u8>>,
    pub stdout_read: Arc<Mutex<Receiver<Vec<u8>>>>,
    stdin_task: Task<()>,
    stdout_task: Task<()>,
    stderr_task: Task<()>,
}

fn spawn_console_menu(
    mut commands: Commands,
    query: Query<Entity, Added<EditorBottomMenu>>,
) {
    for entity in query.iter() {
        #[cfg(target_os = "linux")]
        let mut cmd = Command::new("rev");
        #[cfg(target_os = "windows")]
        let mut cmd = Command::new("cmd");
        let cmd = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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

        let mut stdin = cmd.stdin.take().unwrap();
        let mut stdout = cmd.stdout.take().unwrap();
        let mut stderr = cmd.stderr.take().unwrap();

        let (stdin_write, stdin_read) = channel::<Vec<u8>>();
        let (stdout_write, stdout_read) = channel();

        let pool = AsyncComputeTaskPool::get();

        let stdin_task = pool.spawn(async move {
            while let Ok(bytes) = stdin_read.recv() {
                stdin.write_all(bytes.as_slice()).unwrap();
            }
        });

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
            parent.spawn((Console {
                cmd,
                stdin_write,
                stdout_read: Arc::new(Mutex::new(stdout_read)),
                stdin_task,
                stdout_task,
                stderr_task,
            }, TextBundle::from_section("", TextStyle {
                font: DefaultFonts::ROBOTO_REGULAR,
                ..default()
            })));
        });
    }
}

fn console_stdout(
    mut query: Query<(&mut Text, &Console)>
) {
    for (mut text, console) in query.iter_mut() {
        let mut result = vec![];
        while let Ok(content) = console.stdout_read.lock().unwrap().try_recv() {
            result.extend(content);
        }

        if !result.is_empty() {
            text.sections[0].value.push_str(&*String::from_utf8(result).unwrap());
        } else {
            console.stdin_write.send("echo hello".as_bytes().to_vec()).unwrap();
        }
    }
}
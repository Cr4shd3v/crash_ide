use bevy::prelude::*;
use editor_assets::DefaultFonts;
use editor_widget::{TextInputBundle, TextInputFocused, TextInputSettings, TextInputTextStyle, TextInputValue};
use crate::RawConsole;

pub struct EditorConsolePlugin;

impl Plugin for EditorConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (create_console, console_stdout, console_input))
        ;
    }
}

#[derive(Component)]
pub struct Console;

#[derive(Component)]
pub struct ConsoleInstance {
    raw_console: RawConsole,
    console_output: Entity,
}

#[derive(Component)]
struct ConsoleTextInput;

fn create_console(
    mut commands: Commands,
    query: Query<Entity, Added<Console>>,
) {
    for entity in query.iter() {
        let raw_console = RawConsole::new();

        if let Err(e) = raw_console {
            commands.entity(entity).despawn_descendants().with_children(|parent| {
                parent.spawn(TextBundle::from_section(format!("Could not start console: {}", e), TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }));
            });

            continue;
        }

        let console_output = commands.spawn(TextBundle {
            text: Text::from_section("$", TextStyle {
                font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                font_size: 18.0,
                ..default()
            }),
            style: Style {
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).id();

        let console_input = commands.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }, TextInputBundle {
            text_input_settings: TextInputSettings {
                with_border: false,
                multiline: true,
                ..default()
            },
            text_input_text_style: TextInputTextStyle::default().with_font(DefaultFonts::JETBRAINS_MONO_REGULAR),
            ..default()
        }, ConsoleTextInput)).id();

        commands.entity(entity)
            .insert(ConsoleInstance {
                console_output,
                raw_console: raw_console.unwrap(),
            })
            .despawn_descendants()
            .push_children(&[console_output, console_input]);
    }
}

fn console_stdout(
    mut query: Query<&mut ConsoleInstance>,
    mut text_query: Query<&mut Text>,
) {
    for mut console in query.iter_mut() {
        while let Ok(content) = console.raw_console.try_read() {
            let mut text = text_query.get_mut(console.console_output).unwrap();
            text.sections[0].value.push_str(&*content);
        }
    }
}

fn console_input(
    mut query: Query<(&Parent, &mut TextInputValue), (Changed<TextInputValue>, With<TextInputFocused>, With<ConsoleTextInput>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut console_query: Query<&mut ConsoleInstance>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        for (parent, mut value) in query.iter_mut() {
            let mut console = console_query.get_mut(parent.get()).unwrap();
            let input = std::mem::take(&mut value.0);
            console.raw_console.execute_command(input);
        }
    }
}

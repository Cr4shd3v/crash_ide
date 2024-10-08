use std::path::PathBuf;
use bevy::prelude::*;
use crash_ide_assets::DefaultFonts;
use crash_ide_widget::{Scrollable, ScrollableContent, TextInputBundle, TextInputFocused, TextInputSettings, TextInputTextStyle, TextInputValue};
use crate::RawConsole;

pub struct CrashIDEConsolePlugin;

impl Plugin for CrashIDEConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (create_console, console_stdout, console_input))
        ;
    }
}

#[derive(Component)]
pub struct Console {
    pub start_dir: PathBuf,
}

#[derive(Component)]
pub struct ConsoleInstance {
    raw_console: RawConsole,
    console_output: Entity,
}

#[derive(Component)]
struct ConsoleTextInput;

fn create_console(
    mut commands: Commands,
    query: Query<(Entity, &Console), Added<Console>>,
) {
    for (entity, console) in query.iter() {
        let raw_console = RawConsole::new(&console.start_dir);

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
            text: Text::from_section("", TextStyle {
                font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                font_size: 18.0,
                ..default()
            }),
            style: Style {
                width: Val::Percent(100.0),
                margin: UiRect::horizontal(Val::Px(5.0)),
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
            .insert((ConsoleInstance {
                console_output,
                raw_console: raw_console.unwrap(),
            }, Scrollable::default(), Interaction::None)).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            }, ScrollableContent::default()))
                .push_children(&[console_output, console_input]);
        });
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
    parent_query: Query<&Parent>,
    mut console_query: Query<&mut ConsoleInstance>,
    mut text_query: Query<&mut Text>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        for (parent, mut value) in query.iter_mut() {
            let mut console = console_query.get_mut(parent_query.get(parent.get()).unwrap().get()).unwrap();
            let input = value.0.drain(..).as_str().to_string();
            text_query.get_mut(console.console_output).unwrap().sections[0].value.push_str(&*input);
            console.raw_console.execute_command(format!("{} ; echo $PWD$\n", input.trim()));
        }
    }
}

use bevy::asset::load_internal_binary_asset;
use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::window::PrimaryWindow;

pub(super) struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            CURSOR_HANDLE,
            "../../../assets/Cursor.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );

        app
            .add_event::<TextInputSubmitEvent>()
            .add_systems(Update, (
                create_text_input,
                keyboard,
                update_value.after(keyboard),
                blink_cursor,
                show_hide_cursor,
                update_style,
                show_hide_placeholder,
                focus_text_input,
            ))
        ;
    }
}

const CURSOR_HANDLE: Handle<Font> = Handle::weak_from_u128(0x20e04b82bf39401aaf9ae6a01a7a11b7);

#[derive(Bundle, Default)]
pub struct TextInputBundle {
    pub text_input_value: TextInputValue,
    pub text_input_placeholder: TextInputPlaceholder,
    pub text_input_cursor_timer: TextInputCursorTimer,
    pub text_input_cursor_pos: TextInputCursorPos,
    pub text_input_text_style: TextInputTextStyle,
    pub interaction: Interaction,
    pub text_input_inactive: TextInputInactive,
}

#[derive(Component, Default)]
pub struct TextInputValue(pub String);

#[derive(Component, Default)]
pub struct TextInputPlaceholder {
    pub placeholder: String,
    pub text_style: Option<TextStyle>,
}

#[derive(Component)]
pub struct TextInputCursorTimer {
    pub timer: Timer,
    should_reset: bool,
}

#[derive(Component)]
pub struct TextInputTextStyle(pub TextStyle);

impl Default for TextInputTextStyle {
    fn default() -> Self {
        Self(TextStyle {
            font_size: 18.0,
            ..default()
        })
    }
}

#[derive(Component, Default)]
pub struct TextInputInactive(pub bool);

#[derive(Component, Default)]
pub struct TextInputCursorPos(pub usize);

#[derive(Component, Default)]
pub struct TextInputFocused;

impl Default for TextInputCursorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            should_reset: false,
        }
    }
}

#[derive(Component)]
struct TextInputInner;

#[derive(Component)]
struct TextInputPlaceholderInner;

/// A convenience parameter for dealing with a text input's inner Bevy `Text` entity.
#[derive(SystemParam)]
struct InnerText<'w, 's> {
    text_query: Query<'w, 's, &'static mut Text, With<TextInputInner>>,
    children_query: Query<'w, 's, &'static Children>,
}
impl<'w, 's> InnerText<'w, 's> {
    fn get_mut(&mut self, entity: Entity) -> Option<Mut<'_, Text>> {
        self.children_query
            .iter_descendants(entity)
            .find(|descendant_entity| self.text_query.get(*descendant_entity).is_ok())
            .and_then(|text_entity| self.text_query.get_mut(text_entity).ok())
    }
}

#[derive(Event)]
pub struct TextInputSubmitEvent {
    /// The text input that triggered the event.
    pub entity: Entity,
    /// The string contained in the text input at the time of the event.
    pub value: String,
}

fn keyboard(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut text_input_query: Query<(
        Entity,
        Option<&TextInputFocused>,
        &TextInputInactive,
        &mut TextInputValue,
        &mut TextInputCursorPos,
        &mut TextInputCursorTimer,
    )>,
    mut submit_writer: EventWriter<TextInputSubmitEvent>,
) {
    if events.is_empty() {
        return;
    }

    for (input_entity, focused, inactive, mut text_input,
        mut cursor_pos, mut cursor_timer) in &mut text_input_query
    {
        if inactive.0 || focused.is_none() {
            continue;
        }

        let mut submitted_value = None;

        for event in events.read() {
            if !event.state.is_pressed() {
                continue;
            };

            let pos = cursor_pos.bypass_change_detection().0;

            match event.key_code {
                KeyCode::ArrowLeft => {
                    if pos > 0 {
                        cursor_pos.0 -= 1;

                        cursor_timer.should_reset = true;
                        continue;
                    }
                }
                KeyCode::ArrowRight => {
                    if pos < text_input.0.len() {
                        cursor_pos.0 += 1;

                        cursor_timer.should_reset = true;
                        continue;
                    }
                }
                KeyCode::Backspace => {
                    if pos > 0 {
                        cursor_pos.0 -= 1;
                        text_input.0 = remove_char_at(&text_input.0, cursor_pos.0);

                        cursor_timer.should_reset = true;
                        continue;
                    }
                }
                KeyCode::Delete => {
                    if pos < text_input.0.len() {
                        text_input.0 = remove_char_at(&text_input.0, cursor_pos.0);

                        // Ensure that the cursor isn't reset
                        cursor_pos.set_changed();

                        cursor_timer.should_reset = true;
                        continue;
                    }
                }
                KeyCode::Enter => {
                    submitted_value = Some(text_input.0.clone());
                    commands.entity(input_entity).remove::<TextInputFocused>();

                    continue;
                }
                KeyCode::Tab => {
                    commands.entity(input_entity).remove::<TextInputFocused>();

                    continue;
                }
                KeyCode::Space => {
                    text_input.0.insert(pos, ' ');
                    cursor_pos.0 += 1;

                    cursor_timer.should_reset = true;
                    continue;
                }
                _ => {}
            }

            if let Key::Character(ref s) = event.logical_key {
                let before = text_input.0.chars().take(cursor_pos.0);
                let after = text_input.0.chars().skip(cursor_pos.0);
                text_input.0 = before.chain(s.chars()).chain(after).collect();

                cursor_pos.0 += 1;

                cursor_timer.should_reset = true;
            }
        }

        if let Some(value) = submitted_value {
            submit_writer.send(TextInputSubmitEvent {
                entity: input_entity,
                value,
            });
        }
    }
}

fn update_value(
    mut input_query: Query<
        (
            Entity,
            Ref<TextInputValue>,
            &mut TextInputCursorPos,
        ),
        Or<(Changed<TextInputValue>, Changed<TextInputCursorPos>)>,
    >,
    mut inner_text: InnerText,
) {
    for (entity, text_input, mut cursor_pos) in &mut input_query {
        let Some(mut text) = inner_text.get_mut(entity) else {
            continue;
        };

        // Reset the cursor to the end of the input when the value is changed by
        // a user manipulating the value component.
        if text_input.is_changed() && !cursor_pos.is_changed() {
            cursor_pos.0 = text_input.0.chars().count();
        }

        if cursor_pos.is_changed() {
            cursor_pos.0 = cursor_pos.0.clamp(0, text_input.0.chars().count());
        }

        set_section_values(
            &*text_input.0,
            cursor_pos.0,
            &mut text.sections,
        );
    }
}

fn create_text_input(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &TextInputValue,
        &TextInputTextStyle,
        &TextInputPlaceholder,
        &mut TextInputCursorPos,
        &TextInputInactive,
        Option<&TextInputFocused>,
    ), Added<TextInputValue>>,
    mut style_query: Query<(&mut Style, &mut BorderColor)>
) {
    for (entity, value, style, placeholder,
        mut cursor_pos, inactive, focused) in query.iter_mut() {
        let mut sections = vec![
            // Pre-cursor
            TextSection {
                style: style.0.clone(),
                ..default()
            },
            // cursor
            TextSection {
                style: TextStyle {
                    font: CURSOR_HANDLE,
                    color: if inactive.0 || !focused.is_some() {
                        Color::NONE
                    } else {
                        style.0.color
                    },
                    ..style.0.clone()
                },
                ..default()
            },
            // Post-cursor
            TextSection {
                style: style.0.clone(),
                ..default()
            }
        ];

        cursor_pos.0 = value.0.len();

        set_section_values(&*value.0, cursor_pos.0, &mut sections);

        let text = commands.spawn((
            TextBundle {
                text: Text {
                    linebreak_behavior: BreakLineOn::NoWrap,
                    sections,
                    ..default()
                },
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            },
            TextInputInner,
        )).id();

        let placeholder_style = placeholder.text_style.clone().unwrap_or_else(|| placeholder_style(&style.0));

        let placeholder_visible = inactive.0 && value.0.is_empty();

        let placeholder_text = commands.spawn((
            TextBundle {
                text: Text {
                    linebreak_behavior: BreakLineOn::NoWrap,
                    sections: vec![TextSection::new(&placeholder.placeholder, placeholder_style)],
                    ..default()
                },
                visibility: if placeholder_visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            TextInputPlaceholderInner,
        )).id();

        let overflow_container = commands.spawn(NodeBundle {
            style: Style {
                overflow: Overflow::clip(),
                justify_content: JustifyContent::FlexEnd,
                max_width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).id();

        let (mut style, mut border_color) = style_query.get_mut(entity).unwrap();
        style.border = UiRect::all(Val::Px(1.0));
        border_color.0 = Color::GRAY;

        commands.entity(overflow_container).add_child(text);
        commands.entity(entity).push_children(&[overflow_container, placeholder_text]);
    }
}

fn show_hide_cursor(
    mut input_query: Query<(
        Entity,
        &TextInputTextStyle,
        &mut TextInputCursorTimer,
        &TextInputInactive,
        Option<&TextInputFocused>,
    ), Changed<TextInputInactive>>,
    mut inner_text: InnerText,
    mut removed: RemovedComponents<TextInputFocused>,
) {
    for (entity, style, mut cursor_timer,
        inactive, focused) in input_query.iter_mut() {
        let Some(mut text) = inner_text.get_mut(entity) else {
            continue;
        };

        text.sections[1].style.color = if inactive.0 || focused.is_none() {
            Color::NONE
        } else {
            style.0.color
        };

        cursor_timer.timer.reset();
    }

    for entity in removed.read() {
        let Some(mut text) = inner_text.get_mut(entity) else {
            continue;
        };

        text.sections[1].style.color = Color::NONE;
    }
}

// Blinks the cursor on a timer.
fn blink_cursor(
    mut input_query: Query<(
        Entity,
        &TextInputTextStyle,
        &mut TextInputCursorTimer,
        Ref<TextInputInactive>,
        Option<Ref<TextInputFocused>>,
    )>,
    mut inner_text: InnerText,
    time: Res<Time>,
) {
    for (entity, style, mut cursor_timer, inactive, focused) in &mut input_query {
        if inactive.0 || focused.is_none() {
            continue;
        }

        if cursor_timer.is_changed() && cursor_timer.should_reset {
            cursor_timer.timer.reset();
            cursor_timer.should_reset = false;
            if let Some(mut text) = inner_text.get_mut(entity) {
                text.sections[1].style.color = style.0.color;
            }
            continue;
        }

        if !cursor_timer.timer.tick(time.delta()).just_finished() {
            continue;
        }

        let Some(mut text) = inner_text.get_mut(entity) else {
            continue;
        };

        if text.sections[1].style.color != Color::NONE {
            text.sections[1].style.color = Color::NONE;
        } else {
            text.sections[1].style.color = style.0.color;
        }
    }
}

fn show_hide_placeholder(
    input_query: Query<
        (&Children, &TextInputValue, &TextInputInactive),
        Or<(Changed<TextInputValue>, Changed<TextInputInactive>)>,
    >,
    mut vis_query: Query<&mut Visibility, With<TextInputPlaceholderInner>>,
) {
    for (children, text, inactive) in &input_query {
        let mut iter = vis_query.iter_many_mut(children);
        while let Some(mut inner_vis) = iter.fetch_next() {
            inner_vis.set_if_neq(if text.0.is_empty() && inactive.0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
    }
}

fn update_style(
    mut input_query: Query<(Entity, &TextInputTextStyle), Changed<TextInputTextStyle>>,
    mut inner_text: InnerText,
) {
    for (entity, style) in &mut input_query {
        let Some(mut text) = inner_text.get_mut(entity) else {
            continue;
        };

        text.sections[0].style = style.0.clone();
        text.sections[1].style = TextStyle {
            font: CURSOR_HANDLE,
            ..style.0.clone()
        };
        text.sections[2].style = style.0.clone();
    }
}

fn focus_text_input(
    mut commands: Commands,
    query: Query<(Entity, &Interaction, Option<&TextInputFocused>), (Changed<Interaction>, With<TextInputValue>)>,
    current_focus: Query<Entity, With<TextInputFocused>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let current_focus_entity = current_focus.get_single();
    let mut click_on_text = false;
    let Ok(mut primary_window) = windows.get_single_mut() else { return };

    for (entity, interaction, focused) in query.iter() {
        if *interaction == Interaction::None {
            primary_window.cursor.icon = CursorIcon::Default;
        } else {
            primary_window.cursor.icon = CursorIcon::Text;
        }

        if *interaction != Interaction::Pressed {
            continue;
        }

        click_on_text = true;

        if focused.is_some() {
            continue;
        }

        if let Ok(current_focus_entity) = current_focus_entity {
            commands.entity(current_focus_entity).remove::<TextInputFocused>();
        }

        commands.entity(entity).insert(TextInputFocused);
    }

    if !click_on_text && buttons.any_just_pressed([MouseButton::Left, MouseButton::Right, MouseButton::Middle]) {
        if let Ok(current_focus_entity) = current_focus_entity {
            commands.entity(current_focus_entity).remove::<TextInputFocused>();
        }
    }
}

fn set_section_values(value: &str, cursor_pos: usize, sections: &mut [TextSection]) {
    let before = value.chars().take(cursor_pos).collect();
    let after = value.chars().skip(cursor_pos).collect();

    sections[0].value = before;
    sections[2].value = after;

    if cursor_pos >= value.chars().count() {
        sections[1].value = "}".to_string();
    } else {
        sections[1].value = "|".to_string();
    }
}

fn placeholder_style(style: &TextStyle) -> TextStyle {
    let color = style.color.with_a(style.color.a() * 0.25);
    TextStyle {
        color,
        ..style.clone()
    }
}

fn remove_char_at(input: &str, index: usize) -> String {
    input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if i != index { Some(c) } else { None })
        .collect()
}
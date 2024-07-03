use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use crash_ide_clipboard::Clipboard;
use crate::{CodeViewContent, CodeViewCursorPosition, CodeViewCursorTimer, CodeViewFocused, CodeViewLineContainer, CodeViewStyle};
use crate::update_text::UpdateText;

pub(super) fn keyboard_input(
    mut query: Query<(
        &CodeViewLineContainer,
        &CodeViewStyle,
        &mut CodeViewCursorPosition,
        &mut CodeViewContent,
        &mut CodeViewCursorTimer,
    ), With<CodeViewFocused>>,
    mut events: EventReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    clipboard: Res<Clipboard>,
    mut update_text: UpdateText,
) {
    for (lines, code_view_style, mut cursor_pos,
        mut content, mut timer) in query.iter_mut() {
        for event in events.read() {
            if !event.state.is_pressed() {
                continue;
            };

            match event.key_code {
                KeyCode::ArrowLeft => {
                    if cursor_pos.cursor_pos.x == 0 {
                        if cursor_pos.cursor_pos.y > 0 {
                            cursor_pos.cursor_pos.y -= 1;
                            cursor_pos.cursor_pos.x =
                                content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap() as u32;
                        }
                    } else {
                        cursor_pos.cursor_pos.x -= 1;
                    }
                    timer.reset = true;
                    continue;
                }
                KeyCode::ArrowRight => {
                    cursor_pos.cursor_pos.x += 1;
                    let line_len = content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap() as u32;
                    if cursor_pos.cursor_pos.x > line_len {
                        if content.lines.len() - 1 > cursor_pos.cursor_pos.y as usize {
                            cursor_pos.cursor_pos.y += 1;
                            cursor_pos.cursor_pos.x = 0;
                        } else {
                            cursor_pos.cursor_pos.x -= 1;
                        }
                    }
                    timer.reset = true;
                    continue;
                }
                KeyCode::ArrowUp => {
                    cursor_pos.cursor_pos.y -= 1;
                    let line_len = content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap() as u32;
                    if cursor_pos.cursor_pos.x > line_len {
                        cursor_pos.cursor_pos.x = line_len;
                    }
                    timer.reset = true;
                    continue;
                }
                KeyCode::ArrowDown => {
                    cursor_pos.cursor_pos.y += 1;
                    let line_len = content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap() as u32;
                    if cursor_pos.cursor_pos.x > line_len {
                        cursor_pos.cursor_pos.x = line_len;
                    }
                    timer.reset = true;
                    continue;
                }
                KeyCode::Space => {
                    update_text.insert_text(content.as_mut(), cursor_pos.as_mut(), lines, " ");
                    timer.reset = true;

                    continue;
                }
                KeyCode::Enter => {
                    update_text.insert_new_line(content.as_mut(), cursor_pos.as_mut(), lines, code_view_style);
                    timer.reset = true;

                    continue;
                }
                KeyCode::KeyV => {
                    if keys.pressed(KeyCode::ControlLeft) {
                        let text = clipboard.get_text().unwrap_or(String::new());
                        update_text.insert_text(content.as_mut(), cursor_pos.as_mut(), lines, &*text);
                        timer.reset = true;

                        continue;
                    }
                }
                _ => {}
            }

            if let Key::Character(ref s) = event.logical_key {
                update_text.insert_text(content.as_mut(), cursor_pos.as_mut(), lines, s.as_str());
                timer.reset = true;
            }
        }
    }
}
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use crash_ide_clipboard::Clipboard;
use crate::{TextInputContent, TextInputCursorPosition, TextInputCursorTimer, TextInputFocused, TextInputLineContainer, TextInputStyle};
use crate::update_text::UpdateText;

pub(super) fn keyboard_input(
    mut query: Query<(
        &TextInputLineContainer,
        &TextInputStyle,
        &mut TextInputCursorPosition,
        &mut TextInputContent,
        &mut TextInputCursorTimer,
    ), With<TextInputFocused>>,
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
                    move_cursor_left(cursor_pos.as_mut(), content.as_ref());
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
                KeyCode::Backspace => {
                    let prev_line_len = if cursor_pos.cursor_pos.y > 0 && cursor_pos.cursor_pos.x == 0 {
                        content.get_line_length((cursor_pos.cursor_pos.y - 1) as usize)
                    } else {
                        None
                    };
                    update_text.remove_text(content.as_mut(), cursor_pos.as_mut(), lines, true);
                    move_cursor_left(cursor_pos.as_mut(), content.as_ref());

                    if let Some(prev_line_len) = prev_line_len {
                        cursor_pos.cursor_pos.x = prev_line_len as u32;
                    }

                    timer.reset = true;

                    continue;
                }
                KeyCode::Delete => {
                    update_text.remove_text(content.as_mut(), cursor_pos.as_mut(), lines, false);
                    timer.reset = true;

                    continue;
                }
                KeyCode::Tab => {
                    update_text.insert_text(content.as_mut(), cursor_pos.as_mut(), lines, "    ");
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

fn move_cursor_left(cursor_pos: &mut TextInputCursorPosition, content: &TextInputContent) {
    if cursor_pos.cursor_pos.x == 0 {
        if cursor_pos.cursor_pos.y > 0 {
            cursor_pos.cursor_pos.y -= 1;
            cursor_pos.cursor_pos.x =
                content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap() as u32;
        }
    } else {
        cursor_pos.cursor_pos.x -= 1;
    }
}
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::{CodeViewContent, CodeViewCursorPosition, CodeViewLineContainer, CodeViewStyle, CodeViewToken};
use crate::create::{build_line_command, build_line_count};
use crate::line_container::GetLineContainer;

#[derive(SystemParam)]
pub(crate) struct UpdateText<'w, 's> {
    children_query: Query<'w, 's, &'static Children>,
    text_query: Query<'w, 's, &'static mut Text>,
    get_line_container: GetLineContainer<'w, 's>,
    commands: Commands<'w, 's>,
}

impl<'w, 's> UpdateText<'w, 's> {
    pub fn insert_text(&mut self, content: &mut CodeViewContent, cursor_pos: &mut CodeViewCursorPosition, lines: &CodeViewLineContainer, str: &str) {
        let mut len = 0;
        for (token_index, token) in content.lines[cursor_pos.cursor_pos.y as usize].iter_mut().enumerate() {
            let content_len = token.content.len();
            len += content_len;
            if len >= cursor_pos.cursor_pos.x as usize {
                let index = cursor_pos.cursor_pos.x as usize - (len - content_len);
                token.content.insert_str(index, str);

                let texts_in_line = self.children_query.get(
                    self.get_line_container.get_line(lines, cursor_pos.cursor_pos.y as usize).1,
                ).unwrap();

                let mut text = self.text_query.get_mut(texts_in_line.get(token_index).unwrap().clone()).unwrap();
                text.sections[0].value = token.content.clone();

                cursor_pos.cursor_pos.x += str.len() as u32;

                break;
            }
        }
    }

    pub fn insert_new_line(&mut self, content: &mut CodeViewContent, cursor_pos: &mut CodeViewCursorPosition, lines: &CodeViewLineContainer, code_view_style: &CodeViewStyle) {
        let mut len = 0;
        let mut new_line = String::new();
        for (token_index, token) in content.lines[cursor_pos.cursor_pos.y as usize].iter_mut().enumerate() {
            let content_len = token.content.len();
            len += content_len;
            if len >= cursor_pos.cursor_pos.x as usize {
                let index = cursor_pos.cursor_pos.x as usize - (len - content_len);
                let (first_part, last_part) = token.content.split_at(index);
                new_line = last_part.to_string();
                token.content = first_part.to_string();

                let texts_in_line = self.children_query.get(
                    self.get_line_container.get_line(lines, cursor_pos.cursor_pos.y as usize).1,
                ).unwrap();

                let mut text = self.text_query.get_mut(texts_in_line.get(token_index).unwrap().clone()).unwrap();
                text.sections[0].value = token.content.clone();

                cursor_pos.cursor_pos.x = 0;
                cursor_pos.cursor_pos.y += 1;

                break;
            }
        }

        content.lines.insert(cursor_pos.cursor_pos.y as usize, vec![CodeViewToken {
            content: new_line.to_string(),
            ..default()
        }]);

        let line_entity = build_line_command(&mut self.commands, code_view_style, &content.lines[cursor_pos.cursor_pos.y as usize]);

        self.commands.entity(lines.line_content_container).insert_children(
            cursor_pos.cursor_pos.y as usize + 1,
            &[line_entity],
        );

        self.commands.entity(lines.line_count_container).with_children(|parent| {
            build_line_count(parent, code_view_style, content.lines.len());
        });
    }

    pub fn remove_text(&mut self, content: &mut CodeViewContent, cursor_pos: &mut CodeViewCursorPosition, lines: &CodeViewLineContainer, backspace: bool) {
        let mut len = 0;
        let line_len = content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap();
        let mut line_to_line = None;
        for (token_index, token) in content.lines[cursor_pos.cursor_pos.y as usize].iter_mut().enumerate() {
            let content_len = token.content.len();
            len += content_len;
            if len >= cursor_pos.cursor_pos.x as usize {
                let index = cursor_pos.cursor_pos.x as usize - (len - content_len);
                if cursor_pos.cursor_pos.x == 0 && backspace {
                    if cursor_pos.cursor_pos.y > 0 {
                        // this line to prev line
                        line_to_line = Some((cursor_pos.cursor_pos.y as usize, cursor_pos.cursor_pos.y as usize - 1));
                    }
                } else if cursor_pos.cursor_pos.x as usize == line_len && !backspace {
                    // Next line to this line
                    line_to_line = Some((cursor_pos.cursor_pos.y as usize + 1, cursor_pos.cursor_pos.y as usize));
                } else if index < token.content.len() {
                    token.content.remove(if backspace { index - 1 } else { index });

                    let texts_in_line = self.children_query.get(
                        self.get_line_container.get_line(lines, cursor_pos.cursor_pos.y as usize).1,
                    ).unwrap();

                    let mut text = self.text_query.get_mut(texts_in_line.get(token_index).unwrap().clone()).unwrap();
                    text.sections[0].value = token.content.clone();
                } else {
                    continue;
                }

                break;
            }
        }

        if let Some((from, to)) = line_to_line {
            let from_lines = content.lines.remove(from);
            content.lines[to].extend(from_lines);

            let from_container = self.get_line_container.get_line(lines, from).1;
            let from_texts = self.children_query.get(from_container).unwrap();
            let to_container = self.get_line_container.get_line(lines, to).1;
            self.commands.entity(to_container).push_children(from_texts.as_ref());
            self.commands.entity(from_container).despawn_recursive();
            self.commands.entity(self.get_line_container.get_line(lines, content.lines.len()).0).despawn_recursive();
        }
    }
}
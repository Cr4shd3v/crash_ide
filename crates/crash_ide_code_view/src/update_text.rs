use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::{CodeViewContent, CodeViewContentLine, CodeViewCursorPosition, CodeViewLineContainer, CodeViewStyle, CodeViewToken};
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
        let Some((token_index, insert_index, token)) =
            content.get_line_mut(cursor_pos.cursor_pos.y as usize).unwrap()
            .get_token_mut(cursor_pos.cursor_pos.x as usize) else {
            return;
        };

        token.content.insert_str(insert_index, str);

        let texts_in_line = self.children_query.get(
            self.get_line_container.get_line(lines, cursor_pos.cursor_pos.y as usize).1,
        ).unwrap();

        let mut text = self.text_query.get_mut(texts_in_line.get(token_index).unwrap().clone()).unwrap();
        text.sections[0].value = token.content.clone();

        cursor_pos.cursor_pos.x += str.len() as u32;
    }

    pub fn insert_new_line(&mut self, content: &mut CodeViewContent, cursor_pos: &mut CodeViewCursorPosition, lines: &CodeViewLineContainer, code_view_style: &CodeViewStyle) {
        let Some((token_index, insert_index, token)) =
            content.get_line_mut(cursor_pos.cursor_pos.y as usize).unwrap()
                .get_token_mut(cursor_pos.cursor_pos.x as usize) else {
            return;
        };

        let (first_part, last_part) = token.content.split_at(insert_index);
        let last_part = last_part.to_string();
        token.content = first_part.to_string();

        let texts_in_line = self.children_query.get(
            self.get_line_container.get_line(lines, cursor_pos.cursor_pos.y as usize).1,
        ).unwrap();

        let mut text = self.text_query.get_mut(texts_in_line.get(token_index).unwrap().clone()).unwrap();
        text.sections[0].value = token.content.clone();

        cursor_pos.cursor_pos.x = 0;
        cursor_pos.cursor_pos.y += 1;

        content.lines.insert(cursor_pos.cursor_pos.y as usize, CodeViewContentLine {
            tokens: vec![CodeViewToken {
                content: last_part,
                ..default()
            }],
        });

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
        let mut line_to_line = None;

        if let Some((token_index, insert_index, token)) =
            content.get_line_mut(cursor_pos.cursor_pos.y as usize).unwrap()
                .get_token_mut(cursor_pos.cursor_pos.x as usize + if backspace { 0 } else { 1 }) {
            if cursor_pos.cursor_pos.x == 0 && backspace {
                if cursor_pos.cursor_pos.y > 0 {
                    // this line to prev line
                    line_to_line = Some((cursor_pos.cursor_pos.y as usize, cursor_pos.cursor_pos.y as usize - 1));
                }
            } else if insert_index <= token.content.len() {
                token.content.remove(insert_index - 1);

                let texts_in_line = self.children_query.get(
                    self.get_line_container.get_line(lines, cursor_pos.cursor_pos.y as usize).1,
                ).unwrap();

                let mut text = self.text_query.get_mut(texts_in_line.get(token_index).unwrap().clone()).unwrap();
                text.sections[0].value = token.content.clone();
            }
        } else {
            let line_len = content.get_line_length(cursor_pos.cursor_pos.y as usize).unwrap();
            if cursor_pos.cursor_pos.x as usize == line_len && !backspace {
                // Next line to this line
                line_to_line = Some((cursor_pos.cursor_pos.y as usize + 1, cursor_pos.cursor_pos.y as usize));
            }
        };

        if let Some((from, to)) = line_to_line {
            let from_lines = content.lines.remove(from);
            content.get_line_mut(to).unwrap().tokens.extend(from_lines.tokens);

            let from_container = self.get_line_container.get_line(lines, from).1;
            let from_texts = self.children_query.get(from_container).unwrap();
            let to_container = self.get_line_container.get_line(lines, to).1;
            self.commands.entity(to_container).push_children(from_texts.as_ref());
            self.commands.entity(from_container).despawn_recursive();
            self.commands.entity(self.get_line_container.get_line(lines, content.lines.len()).0).despawn_recursive();
        }
    }
}
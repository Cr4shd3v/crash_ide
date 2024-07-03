use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::{CodeViewContent, CodeViewCursorPosition, CodeViewLineContainer};
use crate::line_container::GetLineContainer;

#[derive(SystemParam)]
pub(crate) struct UpdateText<'w, 's> {
    children_query: Query<'w, 's, &'static Children>,
    text_query: Query<'w, 's, &'static mut Text>,
    get_line_container: GetLineContainer<'w, 's>,
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
}
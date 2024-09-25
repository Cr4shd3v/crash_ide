use std::fmt::Display;
use bevy::prelude::*;

/// Content of a code view
#[derive(Component, Default, Debug)]
pub struct TextInputContent {
    /// All lines
    pub lines: Vec<TextInputContentLine>,
}

impl TextInputContent {
    /// Constructs a [CodeViewContent] from a string without any styling.
    pub fn from_string(string: String) -> Self {
        Self {
            lines: string.split("\n").map(|v| TextInputContentLine {
                tokens: vec![CodeViewToken {
                    content: v.to_string(),
                    ..default()
                }],
            }).collect(),
        }
    }

    /// Get a read only reference to a line by its index
    pub fn get_line(&self, line_index: usize) -> Option<&TextInputContentLine> {
        self.lines.get(line_index)
    }

    /// Get a mutable reference to a line by its index
    pub fn get_line_mut(&mut self, line_index: usize) -> Option<&mut TextInputContentLine> {
        self.lines.get_mut(line_index)
    }

    /// Returns the length of a line or None, if the line does not exist
    pub fn get_line_length(&self, line: usize) -> Option<usize> {
        self.lines.get(line).map(|m| m.len())
    }
}

impl Display for TextInputContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.iter().map(|v| v.tokens.iter().map(|v| v.content.clone()).collect::<String>()).collect::<Vec<String>>().join("\n"))
    }
}

/// Contains information of a line in a [CodeViewContent]
#[derive(Default, Debug)]
pub struct TextInputContentLine {
    /// Tokens contained in this line
    pub tokens: Vec<CodeViewToken>,
}

impl TextInputContentLine {
    /// Returns the total length of the line
    pub fn len(&self) -> usize {
        self.tokens.iter().map(|t| t.content.len()).sum::<usize>()
    }

    /// Finds the corresponding token by a line index.
    ///
    /// Returns the token index, the insert index inside the token and a mutable reference to a token
    pub fn get_token_mut(&mut self, index: usize) -> Option<(usize, usize, &mut CodeViewToken)> {
        let mut len = 0;
        for (token_index, line) in self.tokens.iter_mut().enumerate() {
            let content_len = line.content.len();
            len += content_len;
            if len >= index {
                let inner_token_index = index - (len - content_len);
                return Some((token_index, inner_token_index, line));
            }
        }

        None
    }
}

/// Token describing a part of a line
#[derive(Default, Debug)]
pub struct CodeViewToken {
    /// Content of this token
    pub content: String,
    /// Text should be bold
    pub bold: bool,
    /// Text should be italic
    pub italic: bool,
}
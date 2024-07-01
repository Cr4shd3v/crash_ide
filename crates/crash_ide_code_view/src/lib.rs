#![warn(missing_docs)]
//! Crate implementing the code view of the editor.

mod bundle;
mod create;
mod component;
mod cursor;
mod focus;

use bevy::prelude::*;
pub use bundle::*;
pub use component::*;
use crate::create::create_code_view;
use crate::cursor::{cursor_blinking, init_cursor, update_cursor_pos};
use crate::focus::focus_code_view;

/// Plugin implementing the code view of the editor.
pub struct CrashIDECodeViewPlugin;

impl Plugin for CrashIDECodeViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                create_code_view,
                init_cursor,
                update_cursor_pos,
                cursor_blinking,
                focus_code_view,
            ))
        ;
    }
}
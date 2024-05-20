mod open_project_button;
mod create_project_button;

use bevy::app::App;
use bevy::prelude::{Plugin, Update};
pub(crate) use open_project_button::*;
pub(crate) use create_project_button::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (open_project_button, open_project_directory_picked));
    }
}
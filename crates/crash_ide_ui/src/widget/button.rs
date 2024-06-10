//! This module contains button widgets.

mod open_project_button;
mod create_project_button;
mod github_button;

pub(crate) use bevy::prelude::*;
pub(crate) use open_project_button::*;
pub(crate) use create_project_button::*;
pub(crate) use github_button::*;

pub(super) struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            open_project_button,
            open_project_directory_picked,
            create_project_button,
            open_github_link,
        ));
    }
}
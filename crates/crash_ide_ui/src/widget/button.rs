//! This module contains button widgets.

mod open_project_button;
mod create_project_button;
mod github_button;
mod close_project_button;

use bevy::prelude::*;
pub(crate) use open_project_button::*;
pub(crate) use create_project_button::*;
pub(crate) use github_button::*;
pub(crate) use close_project_button::*;

pub(super) struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            open_project_button,
            open_project_directory_picked,
            create_project_button,
            open_github_link,
            open_github_issue_link,
            close_project_button,
        ));
    }
}
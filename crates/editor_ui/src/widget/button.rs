//! This module contains button widgets.

mod open_project_button;
mod create_project_button;
mod double_click_button;

use bevy::prelude::*;
pub(crate) use open_project_button::*;
pub(crate) use create_project_button::*;
pub(crate) use double_click_button::*;

pub(super) struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            open_project_button,
            open_project_directory_picked,
            create_project_button,
        ));

        app.add_systems(PreUpdate, double_click_detection);
        app.add_systems(PostUpdate, remove_double_click);
    }
}
mod directory;
mod file;

use bevy::prelude::*;
pub use directory::*;
pub use file::*;

pub struct CrashIDEFilePickerPlugin;

impl Plugin for CrashIDEFilePickerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (start_directory_picker, handle_picked_directory, start_file_picker, handle_picked_file))
        ;
    }
}
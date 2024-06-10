use bevy::prelude::{Commands, EventReader};
use editor_widget::ExpandMenuEvent;

pub(super) struct FileMenu;

pub(super) fn spawn_file_menu(
    mut commands: Commands,
    mut event_reader: EventReader<ExpandMenuEvent<FileMenu>>,
) {
    for event in event_reader.read() {

    }
}
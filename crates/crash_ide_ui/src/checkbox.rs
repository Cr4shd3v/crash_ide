use bevy::prelude::*;
use crash_ide_assets::DefaultIcons;
use crash_ide_widget::CheckboxDefaultIcon;

pub(super) fn init_checkbox(
    mut commands: Commands,
    icons: Res<DefaultIcons>,
) {
    commands.insert_resource(CheckboxDefaultIcon(icons.check.clone()));
}
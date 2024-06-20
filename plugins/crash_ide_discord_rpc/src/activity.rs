use bevy::prelude::*;
use bevy::tasks::{block_on, poll_once};
use crash_ide_file::{FileViewInstance, RawOpenFileEvent};
use crash_ide_project::{CloseProjectEvent, LoadedEditorProject, OpenProjectEvent, ProjectRef};
use crash_ide_ui::editor::ProjectsFileViews;
use crash_ide_ui::SwitchProjectWindowEvent;
use crate::client::{DiscordRpcClient, DiscordRpcTask, SetActivityMarker};
use crate::config::DiscordRpcConfig;
use crate::status::DiscordRpcActivity;

pub(super) fn set_project_activity(
    mut event_reader: EventReader<OpenProjectEvent>,
    mut discord_rpc: ResMut<DiscordRpcActivity>,
) {
    for event in event_reader.read() {
        discord_rpc.project = event.crash_ide_project.name.clone();
        discord_rpc.filename = None;
    }
}

pub(super) fn close_project_event(
    mut event_reader: EventReader<CloseProjectEvent>,
    mut discord_rpc_activity: ResMut<DiscordRpcActivity>,
) {
    for _ in event_reader.read() {
        discord_rpc_activity.project = DiscordRpcActivity::SELECT_PROJECT_ACTIVITY.to_string();
        discord_rpc_activity.filename = None;
    }
}

pub(super) fn set_project_activity_switch_window(
    mut event_reader: EventReader<SwitchProjectWindowEvent>,
    mut activity: ResMut<DiscordRpcActivity>,
    query: Query<&LoadedEditorProject>,
    project_file_views: Res<ProjectsFileViews>,
    child_query: Query<&Children>,
    file_view_instance_query: Query<&FileViewInstance>,
) {
    for event in event_reader.read() {
        if let Some(project_entity) = event.new_project {
            let project = query.get(project_entity).unwrap();
            if activity.project == project.crash_ide_project.name {
                continue;
            }
            activity.project = project.crash_ide_project.name.clone();
            let view_entity = project_file_views.get(&ProjectRef(project_entity));
            let Ok(childs) = child_query.get(view_entity) else {
                continue;
            };
            let Ok(file_view_instance) = file_view_instance_query.get(childs[0]) else {
                continue;
            };
            activity.filename = Some(file_view_instance.path.file_name().unwrap().to_str().unwrap().to_string());
        } else {
            activity.project = DiscordRpcActivity::SELECT_PROJECT_ACTIVITY.to_string();
        };
    }
}

pub(super) fn set_filename(
    mut event_reader: EventReader<RawOpenFileEvent>,
    mut activity: ResMut<DiscordRpcActivity>,
) {
    for event in event_reader.read() {
        activity.filename = Some(event.event_data.path.file_name().unwrap().to_str().unwrap().to_string());
    }
}

pub(super) fn trigger_rpc_update(
    mut commands: Commands,
    activity: Res<DiscordRpcActivity>,
    mut discord_rpc: ResMut<DiscordRpcClient>,
    settings: Res<DiscordRpcConfig>,
) {
    if discord_rpc.is_some() {
        let title = activity.project.clone();
        let filename = activity.filename.clone();

        let settings = settings.clone();
        let task = discord_rpc.set_activity(move |act| {
            let mut act = act;

            act = if settings.show_project {
                act.details(title)
            } else {
                act.details(DiscordRpcActivity::DEFAULT_ACTIVITY)
            };

            if settings.show_filename {
                if let Some(filename) = filename {
                    act = act.state(filename);
                }
            }

            act
        });

        commands.spawn(DiscordRpcTask(task));
    }
}

pub(super) fn handle_set_activity(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DiscordRpcTask<SetActivityMarker>)>,
) {
    for (entity, mut task) in query.iter_mut() {
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            if let Err(e) = result {
                println!("Error occurred while setting activity: {}", e);
            }

            commands.entity(entity).despawn();
        }
    }
}

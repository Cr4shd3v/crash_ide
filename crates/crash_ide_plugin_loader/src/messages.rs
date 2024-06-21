use bevy::prelude::*;
use crash_ide_plugin_types::PluginMessage;
use crate::plugin_instance::{LoadedPluginInfo, PluginInstance};

pub(super) struct PluginMessagesPlugin;

impl Plugin for PluginMessagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (parse_plugin_message, plugin_error_messages))
        ;
    }
}

fn parse_plugin_message(
    mut commands: Commands,
    query: Query<(Entity, &PluginInstance, Option<&LoadedPluginInfo>)>,
) {
    for (entity, instance, info) in query.iter() {
        while let Ok(bytes) = instance.try_read() {
            match bincode::decode_from_slice::<PluginMessage, _>(&bytes, bincode::config::standard()) {
                Ok((plugin_message, _)) => {
                    match plugin_message {
                        PluginMessage::PluginInfo(info) => {
                            info!("Plugin {} registered", &info.technical_name);
                            commands.entity(entity).insert(LoadedPluginInfo(info));
                            break;
                        }
                        PluginMessage::PrintLn(msg) => {
                            info!("Plugin {}: {}", info.unwrap().0.technical_name, msg);
                        }
                    }
                },
                Err(e) => {
                    error!("Could not decode plugin message: {}", e);
                },
            }
        }
    }
}

fn plugin_error_messages(
    query: Query<(&PluginInstance, Option<&LoadedPluginInfo>)>,
) {
    for (instance, info) in query.iter() {
        while let Ok(bytes) = instance.try_read_error() {
            if let Some(info) = info {
                error!("Error in plugin {}: {}", &info.0.technical_name, String::from_utf8(bytes).unwrap());
            } else {
                error!("Error in plugin {}: {}", instance.path.file_name().unwrap().to_str().unwrap(), String::from_utf8(bytes).unwrap());
            }
        }
    }
}
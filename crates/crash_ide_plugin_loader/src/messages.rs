use bevy::prelude::*;
use crash_ide_plugin_types::PluginMessage;
use crate::plugin_instance::{LoadedPluginInfo, PluginInstance};

pub(super) struct PluginMessagesPlugin;

impl Plugin for PluginMessagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, parse_plugin_message)
        ;
    }
}

fn parse_plugin_message(
    mut commands: Commands,
    query: Query<(Entity, &PluginInstance, Option<&LoadedPluginInfo>)>,
) {
    for (entity, instance, info) in query.iter() {
        let mut all_bytes = vec![];

        while let Ok(bytes) = instance.try_read() {
            all_bytes.extend(bytes);

            match bincode::decode_from_slice::<PluginMessage, _>(&all_bytes, bincode::config::standard()) {
                Ok((plugin_message, len)) => {
                    all_bytes.drain(0..len);
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

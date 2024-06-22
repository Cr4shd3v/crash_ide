use bevy::prelude::*;
use crash_ide_plugin_types::{PluginboundMessage, ServerboundPluginMessage};
use crate::plugin_instance::{LoadedPluginInfo, PluginInstance};

pub(super) struct PluginMessagesPlugin;

impl Plugin for PluginMessagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, (parse_plugin_info, parse_plugin_message, plugin_error_messages))
            .add_systems(PostUpdate, send_plugin_messages)
            .add_event::<SendPluginMessage>()
            .add_event::<ReceivedPluginMessage>()
        ;
    }
}

#[derive(Event)]
pub struct SendPluginMessage {
    message: PluginboundMessage,
    target_plugin: Option<Entity>,
}

impl SendPluginMessage {
    pub fn new(message: PluginboundMessage, target_plugin: Option<Entity>) -> Self {
        Self {
            message,
            target_plugin,
        }
    }
}

#[derive(Event)]
pub struct ReceivedPluginMessage {
    pub message: ServerboundPluginMessage,
    pub plugin_entity: Entity,
}

fn parse_plugin_info(
    mut commands: Commands,
    query: Query<(Entity, &PluginInstance)>,
) {
    'outer: for (entity, instance) in query.iter() {
        while let Ok(bytes) = instance.try_read() {
            match bincode::decode_from_slice::<ServerboundPluginMessage, _>(&bytes, bincode::config::standard()) {
                Ok((plugin_message, _)) => {
                    match plugin_message {
                        ServerboundPluginMessage::PluginInfo(info) => {
                            info!("Plugin {} registered", &info.technical_name);
                            commands.entity(entity).insert(LoadedPluginInfo(info));
                            continue 'outer;
                        }
                        _ => {
                            error!("Plugin did not send a plugin info!");
                            continue 'outer;
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

fn parse_plugin_message(
    query: Query<(Entity, &PluginInstance, &LoadedPluginInfo)>,
    mut event_writer: EventWriter<ReceivedPluginMessage>,
) {
    for (entity, instance, info) in query.iter() {
        while let Ok(bytes) = instance.try_read() {
            match bincode::decode_from_slice::<ServerboundPluginMessage, _>(&bytes, bincode::config::standard()) {
                Ok((plugin_message, _)) => {
                    match plugin_message {
                        ServerboundPluginMessage::PrintLn(msg) => {
                            info!("Plugin {}: {}", info.0.technical_name, msg);
                        }
                        _ => {
                            event_writer.send(ReceivedPluginMessage {
                                message: plugin_message,
                                plugin_entity: entity,
                            });
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

fn send_plugin_messages(
    mut event_reader: EventReader<SendPluginMessage>,
    instances_query: Query<&PluginInstance>,
) {
    for event in event_reader.read() {
        let data = bincode::encode_to_vec(&event.message, bincode::config::standard()).unwrap();

        if let Some(target) = event.target_plugin {
            instances_query.get(target).unwrap().send(data);
        } else {
            for instance in instances_query.iter() {
                instance.send(data.clone());
            }
        }
    }
}

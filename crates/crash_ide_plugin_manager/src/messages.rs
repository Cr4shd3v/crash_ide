use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crash_ide_plugin_api::{PluginboundMessage, ServerboundPluginMessage, UpdateConfigFields};
use crate::plugin_instance::{LoadedPluginInfo, PluginInstance};

pub(super) struct PluginMessagesPlugin;

impl Plugin for PluginMessagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, (parse_plugin_info, parse_plugin_message, plugin_error_messages))
            .add_systems(PostUpdate, send_plugin_messages)
            .add_event::<SendPluginMessage>()
            .add_event::<ReceivedPluginMessage<UpdateConfigFields>>()
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
pub struct ReceivedPluginMessage<T> {
    pub message: T,
    pub plugin_entity: Entity,
}

fn parse_plugin_info(
    mut commands: Commands,
    query: Query<(Entity, &PluginInstance), Without<LoadedPluginInfo>>,
) {
    'outer: for (entity, instance) in query.iter() {
        while let Ok(bytes) = instance.try_read() {
            match serde_json::from_slice::<ServerboundPluginMessage>(bytes.as_slice()) {
                Ok(plugin_message) => {
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

#[derive(SystemParam)]
struct PluginMessageEventWriter<'w> {
    update_config_fields: EventWriter<'w, ReceivedPluginMessage<UpdateConfigFields>>,
}

fn parse_plugin_message(
    query: Query<(Entity, &PluginInstance, &LoadedPluginInfo)>,
    mut event_writer: PluginMessageEventWriter,
) {
    for (entity, instance, info) in query.iter() {
        while let Ok(bytes) = instance.try_read() {
            match serde_json::from_slice::<ServerboundPluginMessage>(bytes.as_slice()) {
                Ok(plugin_message) => {
                    match plugin_message {
                        ServerboundPluginMessage::PluginInfo(_) => {
                            warn!("Plugin {} sent a second plugin info", info.0.technical_name);
                        }
                        ServerboundPluginMessage::PrintLn(msg) => {
                            info!("Plugin {}: {}", info.0.technical_name, msg.text);
                        }
                        ServerboundPluginMessage::UpdateConfigFields(update_fields) => {
                            event_writer.update_config_fields.send(ReceivedPluginMessage {
                                message: update_fields,
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
        let data = serde_json::to_vec(&event.message).unwrap();

        if let Some(target) = event.target_plugin {
            instances_query.get(target).unwrap().send(data);
        } else {
            for instance in instances_query.iter() {
                instance.send(data.clone());
            }
        }
    }
}

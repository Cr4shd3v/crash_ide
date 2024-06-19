use std::path::PathBuf;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crash_ide_file_watcher::{CreateKind, EventKind, FileWatcherInstance, ModifyKind, RemoveKind, RenameMode};
use crate::editor::editor_left_menu::{FileDisplay, ProjectRoot};

#[derive(SystemParam)]
pub struct RowEntityFromPath<'w, 's> {
    root_query: Query<'w, 's, (&'static ProjectRoot, &'static Children)>,
    query: Query<'w, 's, (Entity, &'static Children, &'static FileDisplay)>,
}

impl<'w, 's> RowEntityFromPath<'w, 's> {
    /// Returns new entry entity id
    pub fn insert_new_entry(&self, commands: &mut Commands, root_entity: Entity, path: &str, is_file: bool) -> Option<Entity> {
        let path_buf = PathBuf::from(path);
        let filename = path_buf.file_name().unwrap().to_str().unwrap().to_string();
        let dir = path.strip_suffix(&*filename).unwrap();

        let row_entity = self.get_row_entity(root_entity, &*dir)?;
        let (_, children, dir_display) = self.query.get(row_entity).unwrap();

        let mut index = 0;
        for (_, _, display) in self.query.iter_many(children) {
            index += 1;

            if !is_file && display.is_file {
                break;
            }

            if is_file && !display.is_file {
                continue;
            }

            if display.filename == filename {
                return None;
            }

            if display.filename > filename {
                break;
            }
        }

        let new_display = commands.spawn(FileDisplay {
            is_file,
            level: dir_display.level + 1,
            filename,
        }).id();

        commands.entity(row_entity).insert_children(index, &[new_display]);

        Some(new_display)
    }

    pub fn get_row_entity(&self, root_entity: Entity, path: &str) -> Option<Entity> {
        let (root, children) = self.root_query.get(root_entity).unwrap();

        let Some(strip) = path.strip_prefix(&*root.full_path) else {
            println!("Path not from this project, {} expected, got {}", root.full_path, path);
            return None;
        };

        let remaining_path = strip.split("/").filter(|v| !v.is_empty()).collect::<Vec<&str>>();

        if remaining_path.is_empty() {
            Some(root_entity)
        } else {
            self.iter_children_for_path(children, remaining_path, 0)
        }
    }

    fn iter_children_for_path(&self, children: &Children, remaining_path: Vec<&str>, index: usize) -> Option<Entity> {
        let search_path = remaining_path.get(index)?;

        for (entity, children, display) in self.query.iter_many(children) {
            if display.filename == *search_path {
                return if remaining_path.len() - 1 == index {
                    Some(entity)
                } else {
                    self.iter_children_for_path(children, remaining_path, index + 1)
                }
            }
        }

        None
    }
}

pub(super) fn handle_file_watcher(
    mut commands: Commands,
    query: Query<(Entity, &FileWatcherInstance), With<ProjectRoot>>,
    row_entity_from_path: RowEntityFromPath,
) {
    for (entity, watcher) in query.iter() {
        while let Ok(event) = watcher.receiver().try_recv() {
            let first_path = event.paths[0].to_str().unwrap().to_string();

            match event.kind {
                EventKind::Create(create) => {
                    match create {
                        CreateKind::File => {
                            // Create file
                            if let None = row_entity_from_path.insert_new_entry(&mut commands, entity, &*first_path, true) {
                                println!("Could not create file from watcher");
                            }
                        }
                        CreateKind::Folder => {
                            // Create folder
                            if let None = row_entity_from_path.insert_new_entry(&mut commands, entity, &*first_path, false) {
                                println!("Could not create folder from watcher");
                            }
                        }
                        _ => {}
                    }
                }
                EventKind::Modify(modify) => {
                    match modify {
                        ModifyKind::Name(name) => {
                            match name {
                                RenameMode::To => {
                                    // Like create file
                                    if let None = row_entity_from_path.insert_new_entry(&mut commands, entity, &*first_path, PathBuf::from(&first_path).is_file()) {
                                        println!("Could not create file from watcher");
                                    }
                                }
                                RenameMode::From => {
                                    // Like delete file
                                    if let Some(row_entity) = row_entity_from_path.get_row_entity(entity, &*first_path) {
                                        commands.entity(row_entity).despawn_recursive();
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                EventKind::Remove(remove) => {
                    match remove {
                        RemoveKind::File | RemoveKind::Folder => {
                            // remove file
                            if let Some(row_entity) = row_entity_from_path.get_row_entity(entity, &*first_path) {
                                commands.entity(row_entity).despawn_recursive();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

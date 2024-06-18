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
    pub fn get_row_entity(&self, root_entity: Entity, path: &str) -> Option<Entity> {
        let (root, children) = self.root_query.get(root_entity).unwrap();

        let Some(strip) = path.strip_prefix(&*root.full_path) else {
            println!("Path not from this project, {} expected, got {}", root.full_path, path);
            return None;
        };

        let remaining_path = strip.split("/").filter(|v| !v.is_empty()).collect::<Vec<&str>>();
        println!("{:?}", remaining_path);

        self.iter_children_for_path(children, remaining_path, 0)
    }

    fn iter_children_for_path(&self, children: &Children, remaining_path: Vec<&str>, index: usize) -> Option<Entity> {
        let search_path = remaining_path[index];

        for (entity, children, display) in self.query.iter_many(children) {
            if display.filename == search_path {
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
                        }
                        CreateKind::Folder => {
                            // Create folder
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

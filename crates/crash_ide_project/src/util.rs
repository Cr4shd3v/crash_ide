use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::LoadedEditorProject;

/// Reference to a [LoadedEditorProject].
#[derive(Component)]
pub struct ProjectRef(pub Entity);

/// System param to obtain a [LoadedEditorProject] from a [ProjectRef]
#[derive(SystemParam)]
pub struct Projects<'w, 's> {
    project_query: Query<'w, 's, &'static LoadedEditorProject>,
}

impl<'w, 's> Projects<'w, 's> {
    /// Obtain a [LoadedEditorProject] from a [ProjectRef]
    pub fn get_by_ref(&self, project_ref: &ProjectRef) -> &LoadedEditorProject {
        self.project_query.get(project_ref.0).unwrap()
    }
}

/// System param to obtain a [LoadedEditorProject] from a ui node
#[derive(SystemParam)]
pub struct FindProjectInParents<'w, 's> {
    query: Query<'w, 's, (&'static Parent, Option<&'static ProjectRef>)>,
    projects: Projects<'w, 's>,
}

impl<'w, 's> FindProjectInParents<'w, 's> {
    /// Find the closest [ProjectRef] in the parents and retrieves the corresponding [LoadedEditorProject]
    pub fn find(&self, entity: Entity) -> &LoadedEditorProject {
        self.projects.get_by_ref(self.find_project_ref(entity))
    }

    /// Find the closest [ProjectRef] in the parents
    pub fn find_project_ref(&self, entity: Entity) -> &ProjectRef {
        let (parent, project_ref) = self.query.get(entity).unwrap();
        if let Some(project_ref) = project_ref {
            project_ref
        } else {
            self.find_project_ref(parent.get())
        }
    }
}
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct ProjectFiles;

impl ProjectFiles {
    pub fn create_config_files(project_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let config_folder = project_path.join(".crash_ide");
        if fs::metadata(&config_folder).is_err() {
            fs::create_dir_all(&config_folder)?;
        }

        let config_gitignore = config_folder.join(".gitignore");
        if fs::metadata(&config_gitignore).is_err() {
            fs::write(config_gitignore, "cache")?;
        }

        let cache_folder = config_folder.join("cache");
        if fs::metadata(&cache_folder).is_err() {
            fs::create_dir_all(&cache_folder)?;
        }

        Ok(())
    }

    pub fn create_new_project_files(project_path: &PathBuf, name: &String) -> Result<(), Box<dyn Error>> {
        Self::create_config_files(project_path)?;

        let readme_file = project_path.join("Readme.md");
        if fs::metadata(&readme_file).is_err() {
            fs::write(readme_file, format!("# {}\n", name)).ok();
        }

        let gitignore_file = project_path.join(".gitignore");
        if fs::metadata(&gitignore_file).is_err() {
            fs::write(gitignore_file, "").ok();
        }

        Ok(())
    }
}
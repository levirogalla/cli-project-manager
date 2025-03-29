mod config_io;
mod env_setup;
mod utils; // re export for tests

use std::{collections::HashSet, fs};
use std::os::unix::fs::symlink;
use std::path::Path;

pub use utils::XDG;
pub use config_io::{Config, Alias, ProjectConfig};

mod api_types {
    pub type AliasName<'a> = &'a str;
    pub type ProjectTypeName<'a> = &'a str;
    pub type LibraryName<'a> = &'a str;
}

pub fn define_project_type(name: &str, default_alias_groups: Option<Vec<String>>, builder: Option<&str>, opener: Option<&str>, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    config.add_project_type(name.to_string(), default_alias_groups, builder, opener);
    config.save(None, xdg).expect("Could not save config");
}

pub fn create_alias_group(name: &str, path: &str, xdg: &XDG) {
    let mut config = Config::load(None, xdg).expect("Could not load config");
    fs::create_dir_all(path).expect("Could not create dir");
    config.add_alias_group(name.to_string(), &Alias::new(path));
    config.save(None, xdg).expect("Could not save config");
}

pub fn create_lib(name: &str, path: &str, default: bool, xdg: &XDG) {
    let mut config = Config::load(None, &xdg).expect("Could not load config");
    fs::create_dir_all(path).expect("Could not create folder");
    config.add_lib(name.to_string(), path, default);
    if default { config.set_default_lib(name.to_string()); }
    config.save(None, xdg).expect("Could not save config");
}


pub fn create_project(name: &str, project_type: Option<api_types::ProjectTypeName>, alias_group: Option<api_types::AliasName>, lib: Option<api_types::LibraryName>, xdg: &XDG) {
    let config = Config::load(None, xdg).expect("Could not load config");
    let project_config_path = Path::new(config.get_lib_path(lib).expect("Could not find lib path")).join(name).join(ProjectConfig::PROJECT_ROOT_REL_PATH);
    let parent_dir = project_config_path.parent().expect("Invalid project config path");

    fs::create_dir_all(parent_dir).expect("Could not create parent directory");
    fs::File::create(&project_config_path).expect("Could not create project config file");
    
    let project_config = ProjectConfig::new(project_type.map(|pt| pt.to_string()), None, None);
    println!("Project config: {:?}", project_config);
    project_config.save(&project_config_path.to_str().unwrap()).expect("Could not save project config");

    let mut project_alias_groups: HashSet<&str> = HashSet::new();
    if let Some(pt) = project_type {
        let project_type_config = config.get_project_type(pt.to_string()).expect("Could not find project type");
        if let Some(alias_groups) = &project_type_config.default_alias_groups {
            project_alias_groups.extend(alias_groups.iter().map(|s| s.as_str()));
        }
    }
    if let Some(ag) = alias_group {
        project_alias_groups.insert(ag);
    }

    for alias_group in project_alias_groups {
        let alias = config.get_alias_group(alias_group).expect("Could not find alias");
        let alias_path = Path::new(&alias.path).join(name);
        symlink(&project_config_path, alias_path).expect("Could not create symlink");
    }


}



pub use env_setup::setup_pm;

use std::{env, fs::copy, path::PathBuf, process::Command};

fn main() {
    let projects = match env::var("PROJECTS") {
        Ok(projects) => PathBuf::from(projects),
        Err(_) => {
            let mut home = PathBuf::from(env::var_os("HOME").unwrap());
            home.push("Projects");
            home
        }
    };
    println!("using projects directory {projects:?}");
    let destination = match env::var("PROJECTS_BIN") {
        Ok(destination) => PathBuf::from(destination),
        Err(_) => {
            let mut home = PathBuf::from(env::var_os("HOME").unwrap());
            home.push("bin");
            home
        }
    };
    println!("using destination directory {destination:?}");
    for project in projects.read_dir().unwrap() {
        let Ok(project) = project else { continue; };
        println!("building {:?}", project.file_name());
        let Ok(result) = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(project.path())
            .status() else { continue; };
        if !result.success() {
            continue;
        }
        println!("built successfully");
        let mut result = project.path();
        result.push("target");
        result.push("release");
        result.push(project.file_name());
        let mut destination = destination.clone();
        destination.push(project.file_name());
        let Ok(_) = copy(result, destination) else { continue; };
        println!("installed {:?}", project.file_name());
    }
}

use std::{
    env, fs, io,
    path::Path,
    process::{Command, Output},
};

use lazy_static::lazy_static;

fn get_home_env() -> String {
    if env::consts::OS == "windows" {
        "USERPROFILE".to_string()
    } else {
        "HOME".to_string()
    }
}

lazy_static! {
    static ref HOME_DIR: String = env::var(get_home_env()).unwrap();
}

pub fn run_command(command: &str, args: Vec<&str>) -> Output {
    let output = Command::new(command).args(args).output().expect("failed");
    output
}

pub fn find_file_in_dir(base_path: &str, name: &str) -> String {
    let mut res: String = String::new();
    let entries = fs::read_dir(&base_path).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if file_name.to_str().unwrap().starts_with(&name)
                    && file_name.to_str().unwrap().ends_with(".gz")
                {
                    res = path.display().to_string()
                }
            }
        }
    }

    res
}

pub fn get_home_dir() -> String {
    let res = &HOME_DIR;
    res.to_string()
}

pub fn get_installation_dir(name: &str) -> String {
    format!("{}/.jvem/{}", get_home_dir(), name)
}

pub fn check_jdk_exists(name: &str) -> bool {
    let d_path = get_installation_dir(name);
    match Path::new(&d_path).exists() {
        true => true,
        false => false,
    }
}

pub fn create_java_dir(name: &str) {
    let new_dir_path = format!("{}/.jvem/{}", *HOME_DIR, name);
    fs::create_dir_all(new_dir_path).unwrap();
}

pub fn check_list_locally() {
    let jvem_dir = format!("{}/.jvem/", get_home_dir());
    let path_dir = Path::new(&jvem_dir);
    match path_dir.exists() {
        true => {
            for entry in fs::read_dir(path_dir).unwrap() {
                let entry = entry.unwrap();
                if entry.file_name() != "java" {
                    println!("{}", entry.file_name().to_str().unwrap());
                }
            }
        }
        false => {
            println!("{}", "no jdk installations found locally");
        }
    }
}

fn is_empty_dir(path: &std::path::Path) -> io::Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}

pub fn clean_jvem() {
    for entry in fs::read_dir(format!("{}/.jvem", get_home_dir())).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() && is_empty_dir(&path).unwrap() {
            fs::remove_dir(&path).unwrap();
        }
    }
}

pub fn extract_tarball_linux(name: String) {
    let tarball_status = run_command(
        "/usr/bin/tar",
        vec![
            "xvzf",
            &find_file_in_dir("/tmp/", &name),
            "--strip-components=1",
            "-C",
            &format!("{}/.jvem/{}", get_home_dir(), name),
        ],
    );

    if tarball_status.status.success() {
        println!("tarball extraction successful ");
    } else {
        println!(
            "tarball extraction failed: {:?} ",
            String::from_utf8_lossy(&tarball_status.stderr)
        );
    }
}

pub fn extract_tarball_macos(temp_directory: &str, name: &str) {
    let tarball_status = run_command(
        "/usr/bin/tar",
        vec![
            "xvzf",
            temp_directory,
            "--strip-components=1",
            "-C",
            &format!("{}/.jvem/{}", get_home_dir(), name),
        ],
    );

    if tarball_status.status.success() {
        println!("tarball extraction successful ");
    } else {
        println!(
            "tarball extraction failed: {:?} ",
            String::from_utf8_lossy(&tarball_status.stderr)
        );
    }
}

pub fn extract_zip(temp_dir: &str, name: &str) {
    let unzip_output = run_command(
        "powershell",
        vec![
            "-Command",
            &format!(
                "Expand-Archive -Path {0} -DestinationPath {1}; mv {1}\\*\\* {1}",
                temp_dir,
                get_installation_dir(name),
            ),
        ],
    );

    if unzip_output.status.success() {
        println!("unzipping successful ");
    } else {
        println!(
            "unzipping failed: {} ",
            String::from_utf8_lossy(&unzip_output.stderr)
        );
    }
}

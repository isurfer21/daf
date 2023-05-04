use std::fs;
use std::path::Path;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use serde_json;
use serde_yaml;
use clap::{App, Arg};

#[derive(Serialize, Deserialize)]
struct FileInfo {
    filename: String,
    r#type: String,
    size: String,
    #[cfg(unix)]
    owner: String,
    #[cfg(unix)]
    group: String,
    mode: String,
    created: DateTime<Local>,
    modified: DateTime<Local>,
}

fn get_file_info(file_name: String, metadata: &fs::Metadata) -> FileInfo {
    let file_type = if metadata.is_dir() { "directory" } else { "file" };
    let file_size = format_size(metadata.len());
    #[cfg(unix)]
    let owner = metadata.uid();
    #[cfg(unix)]
    let group = metadata.gid();
    let mode = get_mode(&metadata);
    let created = DateTime::<Local>::from(metadata.created().unwrap());
    let modified = DateTime::<Local>::from(metadata.modified().unwrap());

    FileInfo {
        filename: file_name,
        r#type: file_type.to_string(),
        size: file_size.to_string(),
        #[cfg(unix)]
        owner: owner.to_string(),
        #[cfg(unix)]
        group: group.to_string(),
        mode,
        created,
        modified,
    }
}

#[cfg(unix)]
fn get_mode(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;
    format!("{:o}", metadata.permissions().mode())
}

#[cfg(windows)]
fn get_mode(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().readonly();
    format!("{}", if mode { "readonly" } else { "writable" })
}

fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut i = 0;
    while size >= 1024. && i < units.len() - 1 {
        size /= 1024.;
        i += 1;
    }
    format!("{:.2} {}", size, units[i])
}

fn list_files_attr(file_path: &Path, ls_format: &str) {
    let metadata = file_path.metadata().unwrap();
    let file_name = file_path.file_name().unwrap().to_os_string().into_string().unwrap();
    let file_info = get_file_info(file_name, &metadata);

    match ls_format {
        "json" => println!("{}", serde_json::to_string_pretty(&file_info).unwrap()),
        "yaml" => println!("{}", serde_yaml::to_string(&file_info).unwrap()),
        "columns" => {
            #[cfg(unix)]
            println!(
                "Filename: {:?}   Size: {}   Type: {}   Mode: {}   Created: {}   Modified: {}   Owner: {}   Group: {}   Mode: {}",
                file_info.filename, file_info.size, file_info.r#type, file_info.mode, file_info.created.format("%Y-%m-%d %H:%M:%S"), file_info.modified.format("%Y-%m-%d %H:%M:%S"), file_info.owner, file_info.group, file_info.mode
            );
            #[cfg(windows)]
            println!("Filename: {:?}   Size: {}   Type: {}   Mode: {}   Created: {}   Modified: {}", file_info.filename, file_info.size, file_info.r#type, file_info.mode, file_info.created.format("%Y-%m-%d %H:%M:%S"), file_info.modified.format("%Y-%m-%d %H:%M:%S"));
        }
        "list" => {
            #[cfg(unix)]
            println!(
                "Filename: {:?}\nSize: {}\nType: {}\nMode: {}\nCreated: {}\nModified: {}\nOwner: {}\nGroup: {}\nMode: {}",
                file_info.filename, file_info.size, file_info.r#type, file_info.mode, file_info.created.format("%Y-%m-%d %H:%M:%S"), file_info.modified.format("%Y-%m-%d %H:%M:%S"), file_info.owner, file_info.group, file_info.mode
            );
            #[cfg(windows)]
            println!("Filename: {:?}\nSize: {}\nType: {}\nMode: {}\nCreated: {}\nModified: {}\n", file_info.filename, file_info.size, file_info.r#type, file_info.mode, file_info.created.format("%Y-%m-%d %H:%M:%S"), file_info.modified.format("%Y-%m-%d %H:%M:%S"));
        }
        "table" => {
            #[cfg(unix)]
            println!("{:<10} {:<10} {:<20} {:<20} {:<10} {:<10} {:<10} {:<10} {:<10} {}", "Type", "Mode", "Created", "Modified", "Owner", "Group", "Mode", "Size", "Filename");
            #[cfg(windows)]
            println!("{:<10} {:<10} {:<20} {:<20} {:<10} {}", "Type", "Mode", "Created", "Modified", "Size", "Filename");
            #[cfg(unix)]
            println!(
                "{:<10} {:<10} {:<20} {:<20} {:<10} {:<10} {:<10} {:<10} {:<10} {:?}",
                file_info.r#type, file_info.mode, file_info.created.format("%Y-%m-%d %H:%M:%S"), file_info.modified.format("%Y-%m-%d %H:%M:%S"), file_info.owner, file_info.group, file_info.mode, file_info.size, file_info.filename
            );
            #[cfg(windows)]
            println!("{:<10} {:<10} {:<20} {:<20} {:<10} {:?}", file_info.r#type, file_info.mode, file_info.created.format("%Y-%m-%d %H:%M:%S"), file_info.modified.format("%Y-%m-%d %H:%M:%S"), file_info.size, file_info.filename);
        }
        _ => eprintln!("Invalid format. Please choose one of json/yaml/columns/list/table."),
    }
}

fn list_files_and_dirs(dir_path: &Path, ls_format: &str) {
    let mut entries = Vec::new();
    match fs::read_dir(dir_path) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().unwrap();
                    let file_name = entry.file_name().into_string().unwrap();
                    entries.push(get_file_info(file_name, &metadata));
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    match ls_format {
        "json" => println!("{}", serde_json::to_string_pretty(&entries).unwrap()),
        "yaml" => println!("{}", serde_yaml::to_string(&entries).unwrap()),
        "columns" => {
            for entry in &entries {
                print!("{}   ", entry.filename);
            }
        }
        "list" => {
            for entry in &entries {
                println!("{}", entry.filename);
            }
        }
        "table" => {
            #[cfg(unix)]
            println!("{:<10} {:<10} {:<20} {:<20} {:<10} {:<10} {:<10} {}", "Type", "Mode", "Created", "Modified", "Size", "Owner", "Group", "Filename");
            #[cfg(windows)]
            println!("{:<10} {:<10} {:<20} {:<20} {:<10} {}", "Type", "Mode", "Created", "Modified", "Size", "Filename");
            for entry in &entries {
                #[cfg(unix)]
                println!("{:<10} {:<10} {:<20} {:<20} {:<10} {:<10} {:<10} {}", entry.r#type, entry.mode, entry.created.format("%Y-%m-%d %H:%M:%S"), entry.modified.format("%Y-%m-%d %H:%M:%S"), entry.size, entry.owner, entry.group, entry.filename);
                #[cfg(windows)]
                println!("{:<10} {:<10} {:<20} {:<20} {:<10} {}", entry.r#type, entry.mode, entry.created.format("%Y-%m-%d %H:%M:%S"), entry.modified.format("%Y-%m-%d %H:%M:%S"), entry.size, entry.filename);
            }
        }
        _ => eprintln!("Invalid format. Please choose one of json/yaml/columns/list/table."),
    }

}

fn main() {
    let matches = App::new("Directories & Files (df)")
        .version("1.0.0")
        .author("Abhishek Kumar <isurfer21@gmail.com>")
        .about("Lists the directory and files with or without attributes as opted.")
        .arg(
            Arg::with_name("PATH")
                .help("Sets the directory/file path to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("json")
                .short('j')
                .long("json")
                .help("Sets the output format to JSON"),
        )
        .arg(
            Arg::with_name("yaml")
                .short('y')
                .long("yaml")
                .help("Sets the output format to YAML"),
        )
        .arg(
            Arg::with_name("columns")
                .short('c')
                .long("columns")
                .help("Sets the output format to columns"),
        )
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .help("Sets the output format to list"),
        )
        .arg(
            Arg::with_name("table")
                .short('t')
                .long("table")
                .help("Sets the output format to table"),
        )
        .get_matches();

    let path = if matches.value_of("PATH").is_none() { "." } else { matches.value_of("PATH").unwrap() };

    let df_ls_fmt;
    if matches.is_present("json") {
        df_ls_fmt = "json";
    } else if matches.is_present("yaml") {
        df_ls_fmt = "yaml";
    } else if matches.is_present("columns") {
        df_ls_fmt = "columns";
    } else if matches.is_present("list") {
        df_ls_fmt = "list";
    } else if matches.is_present("table") {
        df_ls_fmt = "table";
    } else {
        df_ls_fmt = "columns";
    }

    let df_path = Path::new(path);
    
    if df_path.is_dir() {
        println!("Directory: {:?}", df_path);
        list_files_and_dirs(&df_path, &df_ls_fmt);
    } else {
        println!("File: {:?}", df_path);
        list_files_attr(&df_path, &df_ls_fmt);
    }
}

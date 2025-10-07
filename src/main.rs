use std::{fs, process::exit};

use clap::Parser;
use dirs_next::data_dir;

use crate::{
    class::Class,
    cli::{ClassAction, Cli},
    commands::Classes,
};

mod class;
mod cli;
mod commands;

fn main() -> Result<(), String> {
    // find and create the data directory
    let path = match data_dir() {
        None => exit(1),
        Some(mut data_path) => {
            data_path.push("lesson_tracker");
            fs::create_dir_all(&data_path).unwrap();
            data_path.push("classes.json");
            data_path
        }
    };

    // read classes if they exists, otherwise empty
    let mut classes = if path.exists() {
        let classes_string = fs::read_to_string(&path).unwrap();
        let classes_list: Vec<Class> = serde_json::from_str(&classes_string).unwrap();
        Classes(classes_list)
    } else {
        Classes(Vec::new())
    };

    let cli = Cli::parse();

    match cli.name {
        None => classes.list_classes(),
        Some(name) => match cli.action {
            None => classes.list_lessons(&name),
            Some(action) => match action {
                ClassAction::New => classes.new(name),
                ClassAction::Add {
                    lesson_name,
                    done,
                    week,
                } => classes.add(name, lesson_name.join(" "), done, week.unwrap_or(1)),
                ClassAction::Understood { lesson_id } => classes.understood(name, lesson_id),
                ClassAction::Remove { lesson_id } => classes.remove(name, lesson_id),
                ClassAction::Attended { lesson_id } => classes.attended(name, lesson_id),
                ClassAction::Processed { lesson_id } => classes.processed(name, lesson_id),
                ClassAction::Edit { .. } => classes.edit(name, action),
            },
        },
    }?;

    let json_string = serde_json::to_string_pretty(&classes.0).unwrap();
    fs::write(&path, json_string).unwrap();

    Ok(())
}

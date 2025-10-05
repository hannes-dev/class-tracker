use std::fs;

use clap::Parser;

use crate::{
    class::Class,
    cli::{ClassAction, Cli},
    commands::Classes,
};

mod class;
mod cli;
mod commands;

fn main() {
    let classes_string = fs::read_to_string("classes.json").unwrap();
    let classes_list: Vec<Class> = serde_json::from_str(&classes_string).unwrap();
    let mut classes = Classes(classes_list);

    let cli = Cli::parse();

    match cli.name {
        None => classes.list_classes(),
        Some(name) => match cli.action {
            None => classes.list_lessons(name),
            Some(ClassAction::New) => classes.new(name),
            Some(ClassAction::Add { lesson_name }) => classes.add(name, lesson_name.join(" ")),
            Some(ClassAction::Read { lesson_id }) => classes.read(name, lesson_id),
            Some(ClassAction::Attended { lesson_id }) => classes.attended(name, lesson_id),
            Some(ClassAction::Processed { lesson_id }) => classes.processed(name, lesson_id),
        },
    }

    let json_string = serde_json::to_string_pretty(&classes.0).unwrap();
    fs::write("classes.json", json_string).unwrap();
}

use colored::Colorize;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    #[tabled(skip)]
    pub lessons: Vec<Lesson>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self {
            name,
            lessons: Vec::new(),
        }
    }
}

#[derive(Tabled, Default, Serialize, Deserialize)]
#[tabled(display(bool, "display_bool"))]
pub struct Lesson {
    pub name: String,
    #[tabled(rename = "w")]
    pub week: u32,
    pub attended: bool,
    pub understood: bool,
    pub processed: bool,
    pub description: String,
}

impl Lesson {
    pub fn new(name: String, week: u32) -> Self {
        Lesson {
            name,
            week,
            ..Default::default()
        }
    }
}

fn display_bool(val: &bool) -> String {
    if *val { "✅".green() } else { "❌".red() }.to_string()
} // ✔️ ❌ ✅ ❎ ✖️ ☑️ ⭕

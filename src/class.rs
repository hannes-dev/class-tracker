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
    pub attended: bool,
    pub read: bool,
    pub processed: bool,
}

impl Lesson {
    pub fn new(name: String) -> Self {
        Lesson {
            name,
            ..Default::default()
        }
    }
}

fn display_bool(val: &bool) -> String {
    if *val { "✅".green() } else { "❌".red() }.to_string()
} // ✔️ ❌ ✅ ❎ ✖️ ☑️ ⭕

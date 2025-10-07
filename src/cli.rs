// cli.rs

use clap::{Parser, Subcommand};

/// A CLI tool to track class lessons.
#[derive(Parser, Debug)]
#[clap(
    name = "class", // Sets the program name in --help output
    author,
    version,
    about,
    long_about = None
)]
pub struct Cli {
    pub name: Option<String>,

    #[clap(subcommand)]
    pub action: Option<ClassAction>,
}

#[derive(Subcommand, Debug)]
pub enum ClassAction {
    New,

    Add {
        lesson_name: Vec<String>,
        #[clap(short, long)]
        done: Option<String>,
        #[clap(short, long)]
        week: Option<u32>,
    },

    #[clap(alias = "a")]
    Attended {
        lesson_id: usize,
    },

    #[clap(alias = "u")]
    Understood {
        lesson_id: usize,
    },

    #[clap(alias = "rm")]
    Remove {
        lesson_id: usize,
    },

    #[clap(alias = "p")]
    Processed {
        lesson_id: usize,
    },

    #[clap(alias = "e")]
    Edit {
        lesson_id: usize,
        #[clap(short, long)]
        description: Option<Vec<String>>,
        #[clap(short, long)]
        week: Option<u32>,
    },
}

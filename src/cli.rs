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
        #[clap(long)]
        done: Option<String>,
    },

    #[clap(alias = "a")]
    Attended {
        lesson_id: usize,
    },

    #[clap(alias = "r")]
    Read {
        lesson_id: usize,
    },

    #[clap(alias = "p")]
    Processed {
        lesson_id: usize,
    },
}

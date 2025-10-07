// cli.rs

use std::ops::Range;

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
        #[arg(value_parser = parse_number_or_range)]
        lesson_id_range: Range<usize>,
        #[clap(short, long)]
        description: Option<Vec<String>>,
        #[clap(short, long)]
        week: Option<u32>,
    },
}

fn parse_number_or_range(s: &str) -> Result<Range<usize>, String> {
    if let Ok(number) = s.parse() {
        return Ok(number..(number + 1));
    }

    let mut inclusive = false;
    let parts: Vec<&str> = if s.contains("=") {
        inclusive = true;
        s.split("..=").collect()
    } else {
        s.split("..").collect()
    };

    if parts.len() != 2 {
        return Err("Invalid range".to_string());
    }

    let start = parts[0]
        .parse()
        .map_err(|e| format!("Invalid start value: {:?}", e))?;

    let mut end = parts[1]
        .parse()
        .map_err(|e| format!("Invalid end value: {:?}", e))?;
    if inclusive {
        end += 1;
    }

    if start >= end {
        return Err("Start of range must be less than end".to_string());
    }

    dbg!(Ok(start..end))
}

#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use home::home_dir;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn handle_dir(dir: &DirEntry, search: &String) {
    if dir
        .path()
        .to_str()
        .unwrap()
        .ends_with(format!("/{}", search).as_str())
    {
        println!("{}", dir.path().display())
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path.into_os_string().into_string().unwrap();

    let root = home_dir().unwrap();
    WalkDir::new(root)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| handle_dir(&x, path));
    Ok(())
}

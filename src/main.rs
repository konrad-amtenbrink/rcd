#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use home::home_dir;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

struct Dir {
    path: String,
    depth: usize,
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn handle_dir(dir: &DirEntry, search: &String) -> bool {
    return dir
        .path()
        .to_str()
        .unwrap()
        .ends_with(format!("/{}", search).as_str());
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path.into_os_string().into_string().unwrap();

    let root = home_dir().unwrap();
    let mut res = Dir {
        path: "/".to_string(),
        depth: 0,
    };
    for entry in WalkDir::new(root)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
    {
        if !handle_dir(&entry, path) {
            continue;
        }

        if res.path == "/" {
            res.path = entry.path().to_str().unwrap().to_string();
            res.depth = entry.depth();
            continue;
        }

        if res.depth > entry.depth() {
            res.path = entry.path().to_str().unwrap().to_string();
            res.depth = entry.depth();
        }
    }
    println!("{}", res.path);
    Ok(())
}

// Copyright (C) 2026 John Smith
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    convert::AsRef,
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Ignore case for text search
    #[arg(short, long)]
    ignore_case: bool,

    /// Show line number
    #[arg(short, long)]
    line_number: bool,

    /// Text to search in files
    text_to_find: Option<Vec<String>>,
}

fn list_files_in_dir<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut list_files = Vec::new();
    // Only store the paths, not the open ReadDir iterators
    let mut stack = vec![dir.as_ref().to_path_buf()];

    while let Some(current_path) = stack.pop() {
        // Read the directory, handle errors (like permission denied) gracefully
        if let Ok(entries) = fs::read_dir(current_path) {
            entries.flatten().for_each(|entry| {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path.clone());
                }
                // Push all paths, or filter for only files if preferred
                list_files.push(path);
            });
        }
    }
    list_files
}

fn find_matching_line_in_file(
    path: &Path,        // Reference to a path
    text_to_find: &str, // Use &str instead of String for flexibility
) -> Option<HashMap<String, Vec<String>>> {
    let mut matching_lines: HashMap<String, Vec<String>> = HashMap::new();

    // Read file. If error (f.e., file not found), jus return None
    if let Ok(file_content) = fs::read_to_string(path) {
        let mut lines_found = Vec::new();

        file_content.lines().enumerate().for_each(|(i, file_line)| {
            if file_line.contains(text_to_find) {
                lines_found.push(format!("{}: {}", i + 1, file_line));
            }
        });

        // If we found something, add to Map
        if !lines_found.is_empty() {
            let path_str = path.to_string_lossy().into_owned();
            matching_lines.insert(path_str, lines_found);
        }
    }

    if !matching_lines.is_empty() {
        Some(matching_lines)
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();

    match args.text_to_find {
        Some(ref text_to_find) => {
            let files = list_files_in_dir(".");
            let search_pattern = text_to_find.first().unwrap();

            let found_lines: Vec<_> = files
                .par_iter()
                .filter_map(|file| {
                    // Read file, handling potential errors gracefully
                    find_matching_line_in_file(&file, search_pattern)
                })
                .collect();

            found_lines.iter().for_each(|found_line| {
                found_line.par_iter().for_each(|(file_name, lines)| {
                    println!("FILE {}", file_name);
                    println!("{}", "=".to_string().repeat((file_name.len() + 5) * 3));
                    lines.iter().for_each(|line| {
                        let splitted_line: Vec<&str> = line.split(search_pattern).collect();
                        let colored_line =
                            splitted_line.join(&search_pattern.red().bold().to_string());
                        println!("{}", colored_line);
                    });
                    println!();
                    println!();
                })
            });
        }
        None => {
            println!("{}", "No text to search is provided".bold())
        }
    };
}

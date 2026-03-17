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
use std::{
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
    let files = fs::read_dir(dir).unwrap().into_iter();

    let mut dirs_in_stack = vec![files];

    let mut list_files = Vec::new();

    while let Some(files_list) = dirs_in_stack.pop() {
        for file in files_list {
            let filepath = file.unwrap().path();
            if filepath.is_dir() {
                dirs_in_stack.push(fs::read_dir(&filepath).unwrap().into_iter());
            }
            list_files.push(filepath)
        }
    }
    list_files
}

fn find_matching_line_in_file<P: AsRef<Path> + std::fmt::Debug>(
    path_buf: P,
    text_to_find: String,
) -> Option<Vec<String>> {
    let mut matching_lines = vec![];
    if let Ok(file_content) = fs::read_to_string(&path_buf) {
        // Grab the first search term if it exists
        for (i, file_line) in file_content.lines().enumerate() {
            if file_line.contains(&text_to_find) {
                matching_lines.push(format!("{}: {}", i + 1, file_line))
            }
        }
    }

    if matching_lines.len() > 0 {
        Some(matching_lines)
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();

    match args.text_to_find {
        Some(ref text_to_find) => {
            let mut found_lines = vec![];

            // Note: Ensure you have a list_files_in_dir function defined elsewhere
            let files = list_files_in_dir("test");

            for file in files {
                // Read file, handling potential errors gracefully
                let search_pattern = text_to_find.first().unwrap();
                let lines_found_in_file =
                    find_matching_line_in_file(file, search_pattern.to_string());
                if let Some(lines_found) = lines_found_in_file {
                    found_lines.push(lines_found);
                }
            }

            println!("found {:?}", found_lines);
        }
        None => {
            println!("No text to search is provided")
        }
    };
}

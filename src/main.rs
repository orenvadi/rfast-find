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

fn main() {
    let args = Args::parse();
    match args.text_to_find {
        Some(ref text_to_find) => {
            text_to_find.iter().for_each(|arg| print!("{} ", arg));
            println!();
        }
        None => {
            println!("No text to search is provided")
        }
    }
}

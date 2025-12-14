use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'y', long = "year")]
    year: isize,
    #[arg(short = 'd', long = "day")]
    day: isize,
}

const CRATE_TOML_TEMPLATE: &str = r#"[package]
name = "aoc_{year}_{day}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;

const MAIN_RS_TEMPLATE: &str = r#"

fn main() {
    let _input = std::fs::read_to_string("packages/aoc_{year}_{day}/input.txt")
        .expect("Failed to read input file");
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!();
    }
}

"#;

fn main() {
    let args = Args::parse();
    let (year, day) = (args.year, args.day);
    println!("Building AoC {year} Day {day}");

    let crate_name = format!("aoc_{year}_{day}");
    let crate_path = PathBuf::from("packages").join(&crate_name);

    if !crate_path.exists() {
        std::fs::create_dir_all(&crate_path)
            .unwrap_or_else(|e| panic!("Failed to create '{}': '{}'", crate_path.display(), e));
    }

    let input_path = &crate_path.join("input.txt");
    if !input_path.exists() {
        println!("Downloading input to '{}'", input_path.display());
        aoc_utils::download_input_for_day(day as u8, year as u16, input_path)
            .unwrap_or_else(|e| panic!("Failed to download input: {}", e));
    }

    let toml_path = &crate_path.join("Cargo.toml");
    if !toml_path.exists() {
        println!("Creating Cargo.toml at '{}'", toml_path.display());

        std::fs::write(
            toml_path,
            CRATE_TOML_TEMPLATE
                .replace("{year}", &year.to_string())
                .replace("{day}", &day.to_string()),
        )
        .unwrap_or_else(|e| panic!("Failed to write '{}': '{}'", toml_path.display(), e));
    }

    let src_path = &crate_path.join("src");
    if !src_path.exists() {
        std::fs::create_dir_all(src_path)
            .unwrap_or_else(|e| panic!("Failed to create '{}': '{}'", src_path.display(), e));
    }
    let main_rs_path = &src_path.join("main.rs");
    if !main_rs_path.exists() {
        println!("Creating main.rs at '{}'", main_rs_path.display());
        std::fs::write(
            main_rs_path,
            MAIN_RS_TEMPLATE
                .replace("{year}", &year.to_string())
                .replace("{day}", &day.to_string()),
        )
        .unwrap_or_else(|e| panic!("Failed to write '{}': '{}'", main_rs_path.display(), e));
    }
}

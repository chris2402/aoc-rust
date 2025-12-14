use std::path::Path;

fn main() {
    build_utils::download_input_for_day(1, 2025, Path::new("input.txt")).unwrap();
}

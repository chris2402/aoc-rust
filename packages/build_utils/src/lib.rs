use std::path::Path;

fn request_input(day: u8, year: u16) -> anyhow::Result<String> {
    let session_token = std::env::var("AOC_SESSION_TOKEN")?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()?;

    anyhow::Ok(response.text()?)
}

pub fn download_input_for_day(day: u8, year: u16, out: &Path) -> anyhow::Result<()> {
    let input = request_input(day, year)?;
    std::fs::write(out, &input)?;
    anyhow::Ok(())
}

// pub fn create_crate_for_day(_day: u8, _year: u16, _path: &Path) -> anyhow::Result<()> {
//     // Placeholder for crate creation logic
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_requests_input() {
        let day = 1;
        let year = 2023;
        let result = request_input(day, year);

        assert!(result.is_ok());
        let expected = std::fs::read_to_string("src/input_2023_1.txt").unwrap();
        assert!(result.unwrap().lines().eq(expected.lines())); // Compare lines to avoid issues with trailing newlines from OS
    }

    #[test]
    fn it_writes_input_to_file() {
        let day = 1;
        let year = 2023;
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = download_input_for_day(day, year, path);
        assert!(result.is_ok());

        let file_content = std::fs::read_to_string(path).unwrap();
        assert_eq!(file_content, request_input(day, year).unwrap());
    }

    // #[test]
    // fn it_creates_crate_for_day() {
    //     let day = 1;
    //     let year = 2023;
    //     let temp_dir = tempfile::tempdir().unwrap();
    //     let path = temp_dir.path();

    //     let result = create_crate_for_day(day, year, path);
    //     assert!(result.is_ok());

    //     let dir_contents = path.read_dir().expect("Contents of dir was not found");
    //     let dir_contents: Vec<_> = dir_contents.flat_map(|d| d.ok()).collect();

    //     assert!(dir_contents.iter().any(|d| d.file_name() == "Cargo.toml"));
    //     assert!(dir_contents.iter().any(|d| d.file_name() == "src"));
    // }
}

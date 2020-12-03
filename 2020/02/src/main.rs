use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

fn main() -> Result<(), io::Error> {
    let lines = read_lines("./files/passwords.txt")?;

    println!("Result: {:#?}", lines);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    let lines_raw = io::BufReader::new(file).lines();

    lines_raw.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn read_lines_returns_every_line_from_file() {
        let lines = read_lines("./files/example.txt");

        assert_that!(
            lines,
            has(vec![
                "abc".to_string(),
                "bcd".to_string(),
                "cde".to_string()
            ])
        );
    }

    #[test]
    fn read_lines_returns_error_when_file_not_found() {
        let lines = read_lines("./files/example_not_found.txt");

        assert_that!(lines, err());
    }
}

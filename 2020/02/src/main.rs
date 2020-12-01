fn main() {
    println!("{}", text());
}

fn text() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_test() {
        assert_eq!(text(), "Hello, world!");
    }
}

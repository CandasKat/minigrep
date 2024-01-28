use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> String {
    if let Ok(contents) = fs::read_to_string(filename) {
        if contents.contains(query) {
            return format!("'{}' found in {}", query, filename)
        }
    }
    return format!("");
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert_eq!(search_in_file(filename, query), "'rust' found in test.txt");
    }
}
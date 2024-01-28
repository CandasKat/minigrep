use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(filename) {
        if contents.contains(query) {
            return true;
        }
    }
    return false;
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert!(search_in_file(filename, query));
    }
}
pub fn search_in_file(filename: &str, query: &str) -> String {
    format!("")
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
use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(filename) {
        if contents.contains(query) {
            return true;
        }
    }
    return false;
}

pub fn handle_args(args: &[String]) -> String {
    if search_in_file(&args[1], &args[2]) {
        return format!("'{}' found in {}", &args[2], &args[1]);
    }
    return format!("");
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn handle_args_with_valid_args() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust")];
        assert_eq!(handle_args(&args), "'rust' found in test.txt");
    }

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert!(search_in_file(filename, query));
    }


}
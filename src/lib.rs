pub fn search_in_file(filename: &str, query: &str) -> String {
    format!("")
}

pub fn handle_args(args: &[String]) -> String {
    format!("")
}


#[cfg(test)]
mod test {
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
        assert_eq!(search_in_file(filename, query), "'rust' found in test.txt");
    }

    #[test]
    fn not_find_word_in_file() {
        let filename = "test.txt";
        let query = "python";
        assert_eq!(search_in_file(filename, query), "'python' not found in test.txt");
    }
}


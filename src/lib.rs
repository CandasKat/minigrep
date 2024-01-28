use std::fs;

pub enum SearchResult {
    Found,
    NotFound,
    FileEmpty,
    FileReadError,
}
pub fn search_in_file(filename: &str, query: &str) -> SearchResult {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            if contents.is_empty() {
                SearchResult::FileEmpty
            } else if contents.contains(query) {
                SearchResult::Found
            } else {
                SearchResult::NotFound
            }
        }
        Err(_) => SearchResult::FileReadError,
    }
}

// Définition d'une structure pour les arguments
struct Arguments {
    filename: String,
    query: String,
}

// Fonction pour analyser et valider les arguments
fn parse_args(args: &[String]) -> Result<Arguments, String> {
    if args.len() != 3 {
        return Err("Usage: minigrep <filename> <query>".to_string());
    }
    Ok(Arguments {
        filename: args[1].clone(),
        query: args[2].clone(),
    })
}

// La fonction handle_args se concentre maintenant uniquement sur l'exécution de la recherche
pub fn handle_args(args: &[String]) -> String {
    if args.len() != 3 {
        return "Usage: minigrep <filename> <query>".to_string();
    }

    let filename = &args[1];
    let query = &args[2];

    match search_in_file(filename, query) {
        SearchResult::Found => format!("'{}' found in {}", query, filename),
        SearchResult::NotFound => format!("'{}' not found in {}", query, filename),
        SearchResult::FileEmpty => format!("File '{}' is empty", filename),
        SearchResult::FileReadError => format!("Error reading file '{}'", filename),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_args_with_valid_args_find_words() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust")];
        assert_eq!(handle_args(&args), "'rust' found in test.txt");
    }

    #[test]
    fn handle_args_with_valid_args_not_find_words() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("Python")];
        assert_eq!(handle_args(&args), "'Python' not found in test.txt");
    }

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert!(search_in_file(filename, query));
    }

    #[test]
    fn not_find_word_in_file() {
        let filename = "test.txt";
        let query = "python";
        assert!(!search_in_file(filename, query));
    }
}

#[cfg(test)]
mod structural_test {
    use super::*;

    #[test]
    fn test_empty_file() {
        let filename = "empty.txt"; // Assurez-vous que ce fichier existe et est vide
        let query = "rust";

        // Utilisez une correspondance de modèle pour vérifier le résultat
        match search_in_file(filename, query) {
            SearchResult::FileEmpty => (), // Le test réussit si le fichier est vide
            _ => panic!("Expected SearchResult::FileEmpty for an empty file, but got a different result"),
        }
    }

    #[test]
    fn test_argument_count_validation() {
        let insufficient_args = vec![String::from("minigrep")];
        assert_eq!(handle_args(&insufficient_args), "Usage: minigrep <filename> <query>", "Expected usage message for insufficient arguments");

        let excessive_args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust"), String::from("extra")];
        assert_eq!(handle_args(&excessive_args), "Usage: minigrep <filename> <query>", "Expected usage message for excessive arguments");
    }
}
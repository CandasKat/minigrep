use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> bool {
    // Tente de lire le contenu du fichier
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => return false, // Retourne immédiatement false si la lecture échoue
    };

    // Retourne false si le fichier est vide car la requête ne peut pas y être trouvée
    if contents.is_empty() {
        return false;
    }

    // Recherche la requête dans le contenu du fichier et retourne le résultat
    contents.contains(query)
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
    match parse_args(args) {
        Ok(arguments) => {
            let result = search_in_file(&arguments.filename, &arguments.query);
            if result {
                format!("'{}' found in {}", arguments.query, arguments.filename)
            } else {
                format!("'{}' not found in {}", arguments.query, arguments.filename)
            }
        }
        Err(e) => e,
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
        let filename = "empty.txt"; // Make sure this file exists and is empty
        let query = "rust";
        assert!(!search_in_file(filename, query), "Expected false for an empty file, but got true");
    }

    #[test]
    fn test_argument_count_validation() {
        let insufficient_args = vec![String::from("minigrep")];
        assert_eq!(handle_args(&insufficient_args), "Usage: minigrep <filename> <query>", "Expected usage message for insufficient arguments");

        let excessive_args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust"), String::from("extra")];
        assert_eq!(handle_args(&excessive_args), "Usage: minigrep <filename> <query>", "Expected usage message for excessive arguments");
    }

    #[test]
    fn test_file_nonexistent() {
        let filename = "nonexistent.txt";
        let query = "rust";
        assert_eq!(search_in_file(filename, query), false, "Expected false for nonexistent file");
    }
}
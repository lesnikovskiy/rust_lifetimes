fn next_language(languages: &[String], current: &str) -> &str {
    let mut found = false;

    for language in languages {
        if found {
            return language;
        }

        if language == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn main() {
    let languages = vec![String::from("rust"), String::from("go"), String::from("typescript")];

    let result = next_language(&languages, "go");
    println!("{}", result);
}

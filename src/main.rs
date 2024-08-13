fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
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

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang1: &'a str, lang2: &'a str) -> &'a str {
    if lang1.len() > lang2.len() { lang1 } else { lang2 }
}

fn main() {
    let languages = vec![String::from("rust"), String::from("go"), String::from("typescript")];

    let result = next_language(&languages, "go");
    println!("Found language: {}", result);

    let last_lang = last_language(&languages);
    println!("Last language: {}", last_lang);

    let longest = longest_language("go", "typescript");
    println!("The longest language is {}", longest);
}

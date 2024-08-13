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

fn find_keyboard<'a>(keyboards: &'a [String], search_term: &str) -> Option<&'a String> {
    keyboards.iter().find(|k| k.contains(&search_term))
}

fn main() {
    let languages = vec![String::from("rust"), String::from("go"), String::from("typescript")];

    let result = next_language(&languages, "go");
    println!("{}", result);

    let keyboards = vec![
        String::from("Yamaha"),
        String::from("Roland"),
        String::from("Casio"),
        String::from("Moog")
    ];

    match find_keyboard(&keyboards, "Ya") {
        Some(keyboard) => println!("Keyboard found: {}", keyboard),
        None => println!("No keyboard found"),
    }
}

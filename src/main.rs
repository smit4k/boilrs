use std::fs;
use std::io::{self, Write};

fn main() {
    println!("Boilrs --- A boilerplate generator written in Rust");
    println!("Supported languages: Rust, Python, Javascript");
    println!("Enter the language you would want to generate boilerplate code for:");

    let mut lang = String::new();
    io::stdin().read_line(&mut lang).unwrap();
    let lang = lang.trim().to_lowercase();

    let boilerplate = match lang.as_str(){
        "rust" => rust_boilerplate(),
        "python" => python_boilerplate(),
        "javascript" => javascript_boilerplate(),

        _ => {
            println!("Unsupported language");
            return;
        }
        
    };

    println!("Generated boilerplate:\n\n{boilerplate}");
    println!("Would you like to save it as a file? (yes/no)");

    let mut save = String::new();
    io::stdin().read_line(&mut save).unwrap();

    if save.trim().eq_ignore_ascii_case("yes") {
        save_to_file(&lang, &boilerplate);
    }
}

fn rust_boilerplate() -> String {
    r#"fn main() {
    println!("Hello, world!");
}"#
    .to_string()
}

fn python_boilerplate() -> String {
    r#"if __name__ == "__main__":
    print("Hello, world!")"#
        .to_string()
}

fn javascript_boilerplate() -> String {
    r#"console.log("Hello, world!");"#
    .to_string()
}

fn save_to_file(lang: &str, content: &str){
    let filename = format!("boilerplate.{}", match lang {
        "rust" => "rs",
        "python" => "py",
        "javascript" => "js",

        _ => "txt",

    });

    fs::write(&filename, content).expect("Failed to save file");
    println!("Boilerplate was saved to {filename}");
}

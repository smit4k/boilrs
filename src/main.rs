use std::fs;
use std::io::{self};
use std::path::Path;

fn main() {
    println!("Boilrs --- A boilerplate generator written in Rust");
    println!("Supported languages: Rust, C++, Python, Javascript, Java, C#, Go,");
    println!("Enter the language you would want to generate boilerplate code for:");

    let mut lang = String::new();
    io::stdin().read_line(&mut lang).unwrap();
    let lang = lang.trim().to_lowercase();

    let boilerplate = match lang.as_str(){
        "rust" => rust_boilerplate(),
        "python" => python_boilerplate(),
        "javascript" => javascript_boilerplate(),
        "java" => java_boilerplate(),
        "go" => go_boilerplate(),
        "c++" => cpp_boilerplate(),
        "c#" => cs_boilerplate(),

        _ => {
            println!("Unsupported language");
            return;
        }
        
    };

    println!("\nGenerated boilerplate:\n\n{boilerplate}\n");
    println!("Would you like to save it as a file? (yes/y) / no");

    let mut save = String::new();
    io::stdin().read_line(&mut save).unwrap();

    let save = save.trim().to_lowercase();

    if save == "yes" || save == "y" {
        save_to_custom_directory_or_default(&lang, &boilerplate);
    } else {
        println!("Boilerplate was not saved.");
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

fn java_boilerplate() -> String {
    r#"public class Main {
        public static void main(String[] args) {
            System.out.println("Hello, world!");
        }
}"#
    .to_string()
}

fn cs_boilerplate() -> String {
    r#"Console.WriteLine("Hello, world!")"#
    .to_string()
}

fn go_boilerplate() -> String {
    r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}"#
    .to_string()
}

fn cpp_boilerplate() -> String {
    r#"#include <iostream>

int main() {
    std::cout << "Hello, world!" << std::endl;
    return 0;
}
    "#
    .to_string()
}


fn save_to_custom_directory_or_default(lang: &str, content: &str) {
    println!("Enter the directory where you want to save the file (press Enter for current directory):");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).unwrap();
    let directory = directory.trim();

    let path = if directory.is_empty() {
        Path::new(".").to_path_buf()
    } else {
        let path = Path::new(directory);
        if !path.is_dir() {
            println!("Invalid directory. Please make sure the path exists.");
            return;
        }
        path.to_path_buf()
    };

    let filename = format!("boilerplate.{}", match lang {
        "rust" => "rs",
        "python" => "py",
        "javascript" => "js",
        "java" => "java",
        "c#" => "cs",
        "go" => "go",
        "c++" => "cpp",
        _ => "txt",
    });

    let filepath = path.join(filename);

    match fs::write(&filepath, content) {
        Ok(_) => println!("Boilerplate was saved to {}", filepath.display()),
        Err(e) => println!("Failed to save file: {}", e),
    }
}

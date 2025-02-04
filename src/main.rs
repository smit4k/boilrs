use std::fs;
use std::io::{self};
use std::path::Path;

fn main() {
    println!("Boilrs --- A boilerplate generator written in Rust");
    println!(
        "Supported languages: Rust, C, C++, Python, Lua, Javascript, HTML, Java, Kotlin, C#, Go, Ruby, Haskell, Zig"
    );
    println!("Enter the language you would want to generate boilerplate code for:");

    let mut lang = String::new();
    io::stdin().read_line(&mut lang).unwrap();
    let lang = lang.trim().to_lowercase();

    // matching languages to their respective generating boilerplate function
    let boilerplate = match lang.as_str() {
        "rust" => rust_boilerplate(),
        "python" => python_boilerplate(),
        "lua" => lua_boilerplate(),
        "javascript" => javascript_boilerplate(),
        "html" => html_boilerplate(),
        "java" => java_boilerplate(),
        "kotlin" => kotlin_boilerplate(),
        "go" => go_boilerplate(),
        "c" => c_boilerplate(),
        "c++" => cpp_boilerplate(),
        "c#" => cs_boilerplate(),
        "ruby" => ruby_boilerplate(),
        "haskell" => haskell_boilerplate(),
        "zig" => zig_boilerplate(),

        _ => {
            println!("Unsupported language");
            return;
        }
    };

    // Prints the generated boilerplate to CLI
    println!("\nGenerated boilerplate:\n\n{boilerplate}\n");

    println!("Would you like to save it as a file? (yes/y) / no");
    /*
    If the user would like to save the boilerplate as a file,
    if none is provided, it saves to the current directory.
    If user does not want to save to a directory, the code is not saved and the program ends.
    */
    let mut save = String::new();
    io::stdin().read_line(&mut save).unwrap();

    let save = save.trim().to_lowercase();

    if save == "yes" || save == "y" {
        save_to_custom_directory_or_default(&lang, &boilerplate);
    } else {
        println!("Boilerplate was not saved.");
    }
}

// Boilerplate generators
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
    r#"console.log("Hello, world!");"#.to_string()
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
    r#"Console.WriteLine("Hello, world!")"#.to_string()
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

fn ruby_boilerplate() -> String {
    r#"puts "Hello, world!"#.to_string()
}

fn haskell_boilerplate() -> String {
    r#"main = putStrLn "Hello, world!""#.to_string()
}

fn kotlin_boilerplate() -> String {
    r#"fun main(args : Array<String>) {
        println("Hello, world!")
    }"#
    .to_string()
}

fn lua_boilerplate() -> String {
    r#"print("Hello, world!)"#.to_string()
}

fn c_boilerplate() -> String {
    r#"#include <stdio.h>

    int main() {
        printf("Hello, World!\n");
        return 0;
    }"#
    .to_string()
}

fn zig_boilerplate() -> String {
    r#"const std = @import("std");

    pub fn main() !void {
        std.debug.print("Hello, World!\n", .{});
    }"#
    .to_string()
}

fn html_boilerplate() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>HTML 5 Boilerplate</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <script src="index.js"></script>
  </body>
</html>"#
.to_string()
}

/*
Function to save to a specific directory.
Writes to a file under the filename extension associated with the language (c# => .cs, python => .py, etc.)
If path is invalid, it stops.
*/

fn save_to_custom_directory_or_default(lang: &str, content: &str) {
    println!(
        "Enter the directory where you want to save the file (press Enter for current directory):"
    );
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

    let filename = format!(
        "boilerplate.{}",
        match lang {
            "rust" => "rs",
            "python" => "py",
            "lua" => "lua",
            "javascript" => "js",
            "html" => "html",
            "java" => "java",
            "kotlin" => "kt",
            "c#" => "cs",
            "go" => "go",
            "c" => "c",
            "c++" => "cpp",
            "ruby" => "rb",
            "haskell" => "hs",
            "zig" => "zig",
            _ => "txt",
        }
    );

    let filepath = path.join(filename);

    match fs::write(&filepath, content) {
        Ok(_) => println!("Boilerplate was saved to {}", filepath.display()),
        Err(e) => println!("Failed to save file: {}", e),
    }
}

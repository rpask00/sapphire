use colored::*;


pub fn blue(string: impl Into<String>) -> String {
    string.into().blue().to_string()
}

pub fn cyan(string: impl Into<String>) -> String {
    string.into().cyan().to_string()
}

pub fn green(string: impl Into<String>) -> String {
    string.into().green().to_string()
}

pub fn yellow(string: impl Into<String>) -> String {
    string.into().yellow().to_string()
}

pub fn red(string: impl Into<String>) -> String {
    string.into().red().to_string()
}

pub fn bold(string: impl Into<String>) -> String {
    string.into().bold().to_string()
}

pub fn underline(string: impl Into<String>) -> String {
    string.into().underline().to_string()
}

pub fn printc(string: impl Into<String>, f: fn(String) -> String) {
    println!("{}", f(string.into()).as_str());
}

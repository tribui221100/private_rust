use regex::Regex;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub enum AppError{
    RegexError,
    NoFilenameFound,
    NoCodeBlockFound,
    WriteFileError
}

impl From<std::io::Error> for AppError {
    fn from(_err: std::io::Error) -> Self{
        AppError::WriteFileError
    }
}

impl From<regex::Error> for AppError {
    fn from(_err: regex::Error) -> Self{
        AppError::RegexError
    }   
}


pub fn code_generate<'a>(prompt:&'a str, raw_resp:&'a str)
    -> Result <& 'a str, AppError> {
    // 1. Extract the filename using the [FILE: name.rs] marker
    let file_regex = Regex::new(r"\[FILE:\s*(.+?\.rs)\]");
    let file_name : &'a str = match file_regex?.captures(raw_resp) {
        Some(cap) => cap.get(1).unwrap().as_str(),
        None => generate_fallback(prompt),
    };
    // 2. Extract the actual Rust code inside the ```rust ... ``` block
    let code_regex = Regex::new(r"```rust\s*([\s\S]*?)```");
    let code_content = match code_regex?.captures(raw_resp) {
        Some(cap) => cap.get(1).unwrap().as_str().to_string(),
        None => raw_resp.trim().to_string(),
    };
    // 3. Write the extracted code to the file system
    let path = Path::new(&file_name);

    if let Some(parent) = path.parent(){
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(path, code_content)?;

    Ok(file_name)
}

fn generate_fallback(_prompt: &str) -> &'static str {
    "agent_output.rs"
}
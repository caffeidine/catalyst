use crate::debug;
use regex::Regex;
use std::{collections::HashMap, fs, path::Path, sync::OnceLock};

static ENV_REGEX: OnceLock<Regex> = OnceLock::new();
static VAR_REGEX: OnceLock<Regex> = OnceLock::new();

fn env_pattern() -> &'static Regex {
    ENV_REGEX.get_or_init(|| Regex::new(r"\$\{\{([^}]+)\}\}").unwrap())
}

fn var_pattern() -> &'static Regex {
    VAR_REGEX.get_or_init(|| Regex::new(r"\{\{([^}]+)\}\}").unwrap())
}

pub fn replace_variables(input: &str, vars: &HashMap<String, String>) -> String {
    let with_env = env_pattern().replace_all(input, |caps: &regex::Captures| {
        let var_name = &caps[1];
        let value = std::env::var(var_name).unwrap_or_else(|_| caps[0].to_string());
        debug!("ENV var replace: {} => {}", var_name, value);
        value
    });

    var_pattern()
        .replace_all(&with_env, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let value = vars
                .get(var_name)
                .map_or_else(|| caps[0].to_string(), |v| v.to_string());
            debug!("VAR replace: {} => {}", var_name, value);
            value
        })
        .into_owned()
}

pub fn replace_variables_with_files(
    input: &str,
    vars: &HashMap<String, String>,
    test_file_dir: &Path,
) -> Result<String, String> {
    let file_regex = Regex::new(r"\{\{file:([^}]+)\}\}").unwrap();

    let with_files = file_regex.replace_all(input, |caps: &regex::Captures| {
        let file_path = &caps[1];
        match load_file_content(file_path, test_file_dir) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error loading file '{}': {}", file_path, e);
                caps[0].to_string()
            }
        }
    });

    Ok(replace_variables(&with_files, vars))
}

fn load_file_content(file_path: &str, test_file_dir: &Path) -> Result<String, String> {
    if file_path.contains("..") {
        return Err("File path cannot escape test directory".to_string());
    }

    let full_path = test_file_dir.join(file_path);

    if !full_path.exists() {
        return Err(format!("File '{}' does not exist", file_path));
    }

    if !full_path.is_file() {
        return Err(format!("'{}' is not a file", file_path));
    }

    let content = fs::read_to_string(&full_path)
        .map_err(|e| format!("Cannot read file '{}': {}", file_path, e))?;

    Ok(escape_json_string(&content))
}

pub fn escape_json_string(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

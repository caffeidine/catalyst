use regex::Regex;
use std::{collections::HashMap, sync::OnceLock};

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
        std::env::var(&caps[1]).unwrap_or_else(|_| caps[0].to_string())
    });

    var_pattern()
        .replace_all(&with_env, |caps: &regex::Captures| {
            vars.get(&caps[1])
                .map_or_else(|| caps[0].to_string(), |v| v.to_string())
        })
        .into_owned()
}

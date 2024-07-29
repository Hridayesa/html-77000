use std::env::args;
use std::fs::read_to_string;

use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, warn};

static DEFAULT_CONFIG_NAME: &str = "config.toml";

lazy_static! {
    pub static ref CONFIG: Config = Config::load_config();
}

#[derive(Debug, Error)]
pub enum CfgError {
    #[error("Ошибка ввода вывода")]
    Io { #[from]source: std::io::Error },
    #[error("Ошибка разбора Toml")]
    Toml { #[from]source: toml::de::Error },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub thread_num: u32,
    pub template_pattern: String,
    pub poem_template: String,
    pub problem_template: String,
    pub src_dir: String,
    pub res_dir: String,
}

impl Config {
    fn load_config() -> Config {
        let args = args().collect::<Vec<String>>();
        let config_name = match args.len() {
            2 => {
                args[1].as_str()
            }
            _ => DEFAULT_CONFIG_NAME
        };
        let res = Config::do_load_parse(config_name);
        info!("{:?}", res);
        res.unwrap_or_else(|e| {
            warn!("Error of parsing config file:{:?} ",e);
            Config::default()
        })
    }

    fn do_load_parse(config_name: &str) -> Result<Config, CfgError> {
        let config_text = read_to_string(config_name)?;
        Ok(toml::from_str(config_text.as_str())?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thread_num: 100,
            template_pattern: "templates/**/*".to_string(),
            poem_template: "poems_77000.html".to_string(),
            problem_template: "problems.html".to_string(),
            src_dir: "data/src".to_string(),
            res_dir: "data/res".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_text() -> String {
        String::from(r#"
            thread_num = 100
            template_pattern = 'templates/**/*'
            poem_template = 'poem'
            problem_template = 'problem'
            src_dir = 'src'
            res_dir = 'res'
        "#)
    }

    #[test]
    fn test_default_config() {
        assert_eq!(100, CONFIG.thread_num);
        assert_eq!("templates/**/*".to_string(), CONFIG.template_pattern);
        assert_eq!("poems_77000.html".to_string(), CONFIG.poem_template);
        assert_eq!("problems.html".to_string(), CONFIG.problem_template);
        assert_eq!("data/src".to_string(), CONFIG.src_dir);
        assert_eq!("data/res".to_string(), CONFIG.res_dir);
    }

    #[test]
    fn test() {
        let config: Config = toml::from_str(get_test_text().as_str()).unwrap();
        println!("{:#?}", config);
    }
}

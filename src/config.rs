use std::{fs::File, path::PathBuf, io::Read};

use crate::err::LauncherError;

pub(crate) mod objects {
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize, Debug)]
    pub(crate) struct Root {
        pub browser: HashMap<String, Browser>,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct Browser {
        #[serde(default = "default_priority")]
        pub priority: i32,
        pub path: String,
        #[serde(default = "default_args")]
        pub args: Vec<String>,
        pub matching: Vec<Match>,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct Match {
        pub pattern: String,
        #[serde(default = "default_kind")]
        pub kind: MatchKind,
        #[serde(default = "default_include_path")]
        pub include_path: bool,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) enum MatchKind {
        SimpleMatch,
        Regex
    }

    fn default_args() -> Vec<String> {
        vec![]
    }

    fn default_priority() -> i32 {
        0
    }

    fn default_kind() -> MatchKind {
        MatchKind::SimpleMatch
    }
    
    fn default_include_path() -> bool {
        false
    }
}

pub(crate) fn read_config() -> Result<objects::Root, LauncherError> {
    let file_path = std::env::current_exe()?
    .parent()
    .unwrap()
    .join("config.toml");
    let config_root = parse_file(file_path)?;

    Ok(config_root)
}

fn parse_file(file_path: PathBuf) -> Result<objects::Root, LauncherError> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let value: objects::Root = toml::from_str(&content)
        .map_err(|source| LauncherError::Config { source })?;
    Ok(value)
}

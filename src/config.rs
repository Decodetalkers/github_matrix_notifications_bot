use std::{io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub botname: String,
    pub keys: Keys,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Keys {
    pub matrix_acount: String,
    pub matrix_passward: String,
    pub homeserver: String,
    pub rooms: Option<Vec<String>>,
}

impl Config {
    pub fn config_from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let Ok(mut file) = std::fs::OpenOptions::new()
            .read(true)
            .open(path)
        else {
            return None
        };
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            return None;
        };
        toml::from_str(&buf).unwrap_or(None)
    }
}

#[test]
fn tst_toml() {
    let config_src = include_str!("../misc/setting.toml");
    let config: Config = toml::from_str(config_src).unwrap();
    assert_eq!(
        config,
        Config {
            botname: "hello".to_string(),
            keys: Keys {
                matrix_acount: "youraccountfullname".to_string(),
                matrix_passward: "yourpassward".to_string(),
                homeserver: "https://matrix-client.matrix.org".to_string(),
                rooms: Some(vec!["a".to_string(), "b".to_string()]),
            }
        }
    );
}

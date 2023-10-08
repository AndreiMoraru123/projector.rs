use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::Config;

type HM = HashMap<PathBuf, HashMap<String, String>>;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HM,
}

struct Projector {
    config: Config,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut paths = vec![];
        let mut curr = Some(self.config.pwd.as_path());

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent()
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter());
            }
        }

        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            curr = p.parent()
        }

        return out;
    }
    pub fn from_config(config: Config) -> Self {
        let config_path = &config.config;
        let data = if std::fs::metadata(config_path).is_ok() {
            let contents = std::fs::read_to_string(&config.config);
            let contents = contents.unwrap_or(String::from("{\"projector\":{}}"));
            serde_json::from_str(&contents).unwrap_or(default_data())
        } else {
            default_data()
        };

        return Projector {
            config,
            data,
        };
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data.projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: String) {
        self.data.projector
            .get_mut(&self.config.pwd)
            .map(|x| {
                x.remove(&key);
            });
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use collection_macros::hashmap;
    use crate::config::Config;
    use crate::projector::{Data, HM, Projector};

    fn get_data() -> HM {
        return hashmap! {
            PathBuf::from("/") => hashmap! {
                "foo".into() => "bar".into(),
                "fem".into() => "is_great".into(),
            },
            PathBuf::from("/foo") => hashmap! {
                "foo".into() => "bar2".into(),
            },
            PathBuf::from("/foo/bar") => hashmap!   {
                "foo".into() => "bar3".into(),
            },
        };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: Config {
                pwd,
                config: PathBuf::from(""),
                operation: crate::config::Operation::Print(None)
            },
            data: Data {
                projector: get_data()
            }
        }
    }

    #[test]
    fn get_value() {
        let proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(String::from("bar3")).as_ref());
        assert_eq!(proj.get_value("fem"), Some(String::from("is_great")).as_ref());
    }

    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.set_value(String::from("foo"), String::from("bar4"));
        proj.set_value(String::from("fem"), String::from("is_better_than_great"));

        assert_eq!(proj.get_value("foo"), Some(String::from("bar4")).as_ref());
        assert_eq!(proj.get_value("fem"), Some(String::from("is_better_than_great")).as_ref());
    }

    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.remove_value(String::from("foo"));
        proj.remove_value(String::from("fem"));

        assert_eq!(proj.get_value("foo"), Some(String::from("bar2")).as_ref());
        assert_eq!(proj.get_value("fem"), Some(String::from("is_great")).as_ref());
    }

}
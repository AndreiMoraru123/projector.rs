use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};
use crate::opts::Opts;


#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;

        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!("operation add expects 2 arguments but got {}", value.len() - 1));
            }

            let mut drain = value.drain(1..=2);

            return Ok(
                Operation::Add(
                    drain.next().expect("to exist"),
                    drain.next().expect("to exist"),
                )
            )
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!("operation rm expects 1 arguments but got {}", value.len() - 1));
            }

            let arg = value.pop().expect("to exist");

            return Ok(
                Operation::Remove(arg)
            )
        }

        if value.len() > 1 {
            return Err(anyhow!("operation print expects 1 arguments but got {}", value.len()));
        }
        let arg = value.pop().expect("to exist");

        return Ok(
            Operation::Print(Some(arg))
        )
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::var("APPDATA").context("unale to get APPDATA")?;
    let mut loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");
    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("errored getting current_dir")?);
}

#[derive(Debug, Clone)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_config(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}


#[cfg(test)]
mod test {
    use anyhow::Result;
    use crate::config::{Config, Operation};
    use crate::opts::Opts;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));
        return Ok(());
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["foo".to_string()],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some("foo".to_string())));
        return Ok(());
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let opts: Config = Opts {
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Add(
            String::from("foo"),
            String::from("bar")));

        return Ok(());
    }

    #[test]
    fn test_remove_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec![
                String::from("rm"),
                String::from("foo"),
            ],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Remove(
            String::from("foo")));

        return Ok(());
    }
}
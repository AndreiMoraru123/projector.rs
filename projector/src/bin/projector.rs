use clap::Parser;
use src::{opts::Opts, config::Config, projector::Projector};

use anyhow::Result;
use src::config::Operation;

fn main () -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut proj = Projector::from_config(config.clone());

    match &config.operation {
        Operation::Add(k, v) => {
            proj.set_value(k.clone(), v.clone());
            proj.save()?;
        },
        Operation::Print(Some(k)) => {
            if k.is_empty() {
                // do nothing
            } else {
                match proj.get_value(k) {
                    Some(v) => println!("{}", v),
                    None => eprintln!("key {:?} not found", k),
                }
            }
        }
        Operation::Remove(k) => {
            proj.remove_value(k.clone());
            proj.save().expect("TODO: panic message");
        }
        _ => {}
    }
    return Ok(());
}
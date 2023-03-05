use std::process;

use anyhow::Result;
use simple_logger::SimpleLogger;
use voice2keys::{Config, Listener};

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(err) => {
            log::error!("{:?}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    // TODO: Add an option to specify this on the command line.
    let config = Config::new("voice2keys.toml".to_owned())?;
    let listener = Listener::new(config)?;

    listener.listen_and_block()?;

    Ok(())
}

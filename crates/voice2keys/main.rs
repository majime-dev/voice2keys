use anyhow::Result;
use core::{Config, Listener};
use simple_logger::SimpleLogger;
use std::process;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(err) => {
            log::error!("{:?}", err);

            // Let's give the user a chance to see the error message if they double-clicked the binary.
            terminal::pause("Press any key to exit...");
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

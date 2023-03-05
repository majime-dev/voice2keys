use anyhow::{bail, Context, Result};
use config::Config as ConfigRepo;
use enigo::Key;
use serde::Deserialize;
use std::collections::HashMap;
use std::time;

#[derive(Debug, Deserialize)]
// Timing represents the configuration for timing between various aspects of the program.
pub struct Timing {
    // How long to wait between key presses.
    pub key_sequence_delay: time::Duration,
    // How long to wait between a key down and a key up event.
    pub key_hold_duration: time::Duration,
}

#[derive(Debug, Deserialize)]
// Config represents a fully-formed, validated configuration.
pub struct Config {
    // A map of voice commands to a sequence of keys to be pressed.
    pub commands: HashMap<String, Vec<Key>>,
    pub timing: Timing,
}

impl Config {
    pub fn new(path: String) -> Result<Self> {
        log::debug!("Loading config from {}", path);

        let config = ConfigRepo::builder()
            .add_source(config::File::with_name(path.as_str()))
            .build()
            .with_context(|| format!("failed to read config from {}", path))?
            .try_deserialize::<Config>()
            .with_context(|| format!("failed to parse config from {}", path))?;

        log::debug!("Loaded config from {}: {:#?}", path, config);

        if config.commands.is_empty() {
            bail!("commands cannot be empty");
        }

        if config.timing.key_hold_duration.is_zero() {
            bail!("key_hold_duration cannot be empty");
        }

        if config.timing.key_sequence_delay.is_zero() {
            log::warn!("key_sequence_delay is set to 0, this is probably not what you want. No delay between keypresses will cause some programs to skip presses following the first one.");
        }

        Ok(config)
    }
}

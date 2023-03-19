use anyhow::{bail, Context, Result};
use config::Config as ConfigRepo;
use input::Vk;
use serde::{de::IntoDeserializer, Deserialize, Deserializer};
use std::collections::HashMap;
use std::time;

#[derive(Debug, Default, Deserialize)]
// Timing represents the configuration for timing between various aspects of the program.
pub struct Timing {
    // How long to wait between key presses.
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub sequence_delay: time::Duration,
    // How long to wait between a key down and a key up event.
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub hold_duration: time::Duration,
}

#[derive(Debug, Deserialize)]
// Config represents a fully-formed, validated configuration.
pub struct Config {
    // A map of voice commands to a sequence of keys to be pressed.
    #[serde(deserialize_with = "parse_sequence")]
    pub commands: HashMap<String, Vec<Vk>>,
    #[serde(default)]
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

        Ok(config)
    }
}

fn parse_sequence<'de, D>(deserializer: D) -> Result<HashMap<String, Vec<Vk>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut parsed: HashMap<String, Vec<Vk>> = Default::default();
    let commands: HashMap<String, String> = Deserialize::deserialize(deserializer)?;

    for (command, sequence) in commands {
        let sequence = sequence
            .split('+')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        parsed.insert(
            command,
            Deserialize::deserialize(sequence.into_deserializer())?,
        );
    }

    Ok(parsed)
}

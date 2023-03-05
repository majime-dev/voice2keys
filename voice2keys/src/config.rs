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
    #[serde(with = "humantime_serde")]
    pub key_sequence_delay: time::Duration,
    // How long to wait between a key down and a key up event.
    #[serde(with = "humantime_serde")]
    pub key_hold_duration: time::Duration,
}

#[derive(Debug)]
// Config represents a fully-formed, validated configuration.
pub struct Config {
    // A map of voice commands to a sequence of keys to be pressed.
    pub commands: HashMap<String, Vec<Key>>,
    pub timing: Timing,
}

#[derive(Debug, Deserialize)]
// RawConfig needs to have its key sequences parsed.
struct RawConfig {
    // A map of voice commands to a sequence of keys to be pressed.
    pub commands: HashMap<String, String>,
    pub timing: Timing,
}

impl Config {
    pub fn new(path: String) -> Result<Self> {
        log::debug!("Loading config from {}", path);

        let raw = ConfigRepo::builder()
            .add_source(config::File::with_name(path.as_str()))
            .build()
            .with_context(|| format!("failed to read config from {}", path))?
            .try_deserialize::<RawConfig>()
            .with_context(|| format!("failed to parse config from {}", path))?;

        log::debug!("Loaded config from {}: {:#?}", path, raw);

        // Let's perform the cheap checks first.
        if raw.commands.is_empty() {
            bail!("commands cannot be empty");
        }

        if raw.timing.key_hold_duration.is_zero() {
            bail!("key_hold_duration cannot be empty");
        }

        if raw.timing.key_sequence_delay.is_zero() {
            log::warn!("key_sequence_delay is set to 0, this is probably not what you want. No delay between keypresses will cause some programs to skip presses following the first one.");
        }

        let mut config = Self {
            commands: HashMap::new(),
            timing: raw.timing,
        };

        for (command, sequence) in raw.commands {
            let sequence = parse_sequence(sequence.as_str())
                .with_context(|| format!("failed to parse {} as a key sequence", sequence))?;

            config.commands.insert(command, sequence);
        }

        Ok(config)
    }
}

// TODO: This could be extended to accept a DSL of sorts...
fn parse_sequence(sequence: &str) -> Result<Vec<Key>> {
    let sequence = sequence
        .split('+')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let mut parsed = Vec::new();

    for key in sequence {
        parsed.push(parse_key(key).with_context(|| format!("failed to parse {} as a key", key))?);
    }

    Ok(parsed)
}

fn parse_key(key: &str) -> Result<Key> {
    let key = match key {
        "Alt" => Key::Alt,
        "Backspace" => Key::Backspace,
        "CapsLock" => Key::CapsLock,
        "Control" => Key::Control,
        "Delete" => Key::Delete,
        "DownArrow" => Key::DownArrow,
        "End" => Key::End,
        "Escape" => Key::Escape,
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        "Home" => Key::Home,
        "LeftArrow" => Key::LeftArrow,
        "Meta" => Key::Meta,
        "PageDown" => Key::PageDown,
        "PageUp" => Key::PageUp,
        "Return" => Key::Return,
        "RightArrow" => Key::RightArrow,
        "Shift" => Key::Shift,
        "Space" => Key::Space,
        "Tab" => Key::Tab,
        "UpArrow" => Key::UpArrow,
        _ => {
            if key.len() == 1 {
                Key::Layout(key.chars().next().unwrap())
            } else {
                bail!("unknown key: {}", key);
            }
        }
    };

    Ok(key)
}

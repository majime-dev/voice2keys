use anyhow::{bail, Context, Result};
use config::Config as ConfigRepo;
use input::Vk;
use serde::Deserialize;
use std::collections::HashMap;
use std::time;

#[derive(Debug, Default, Deserialize)]
// Timing represents the configuration for timing between various aspects of the program.
pub struct Timing {
    // How long to wait between key presses.
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub key_sequence_delay: time::Duration,
    // How long to wait between a key down and a key up event.
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub key_hold_duration: time::Duration,
}

#[derive(Debug)]
// Config represents a fully-formed, validated configuration.
pub struct Config {
    // A map of voice commands to a sequence of keys to be pressed.
    pub commands: HashMap<String, Vec<Vk>>,
    pub timing: Timing,
}

#[derive(Debug, Deserialize)]
// RawConfig needs to have its key sequences parsed.
struct RawConfig {
    // A map of voice commands to a sequence of keys to be pressed.
    pub commands: HashMap<String, String>,
    #[serde(default)]
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
fn parse_sequence(sequence: &str) -> Result<Vec<Vk>> {
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

fn parse_key(key: &str) -> Result<Vk> {
    let key = match key {
        "Alt" => Vk::Alt,
        "Backspace" => Vk::Backspace,
        "CapsLock" => Vk::CapsLock,
        "Control" => Vk::Control,
        "Delete" => Vk::Delete,
        "DownArrow" => Vk::DownArrow,
        "End" => Vk::End,
        "Escape" => Vk::Escape,
        "F1" => Vk::F1,
        "F2" => Vk::F2,
        "F3" => Vk::F3,
        "F4" => Vk::F4,
        "F5" => Vk::F5,
        "F6" => Vk::F6,
        "F7" => Vk::F7,
        "F8" => Vk::F8,
        "F9" => Vk::F9,
        "F10" => Vk::F10,
        "F11" => Vk::F11,
        "F12" => Vk::F12,
        "Home" => Vk::Home,
        "LeftArrow" => Vk::LeftArrow,
        "Windows" => Vk::LeftWin,
        "PageDown" => Vk::PageDown,
        "PageUp" => Vk::PageUp,
        "Enter" => Vk::Enter,
        "RightArrow" => Vk::RightArrow,
        "Shift" => Vk::Shift,
        "Space" => Vk::Space,
        "Tab" => Vk::Tab,
        "UpArrow" => Vk::UpArrow,
        "0" => Vk::_0,
        "1" => Vk::_1,
        "2" => Vk::_2,
        "3" => Vk::_3,
        "4" => Vk::_4,
        "5" => Vk::_5,
        "6" => Vk::_6,
        "7" => Vk::_7,
        "8" => Vk::_8,
        "9" => Vk::_9,
        "A" => Vk::A,
        "B" => Vk::B,
        "C" => Vk::C,
        "D" => Vk::D,
        "E" => Vk::E,
        "F" => Vk::F,
        "G" => Vk::G,
        "H" => Vk::H,
        "I" => Vk::I,
        "J" => Vk::J,
        "K" => Vk::K,
        "L" => Vk::L,
        "M" => Vk::M,
        "N" => Vk::N,
        "O" => Vk::O,
        "P" => Vk::P,
        "Q" => Vk::Q,
        "R" => Vk::R,
        "S" => Vk::S,
        "T" => Vk::T,
        "U" => Vk::U,
        "V" => Vk::V,
        "W" => Vk::W,
        "X" => Vk::X,
        "Y" => Vk::Y,
        "Z" => Vk::Z,
        _ => {
            // TODO: Support lowercase keys in Config
            bail!("unknown key: {}", key);
        }
    };

    Ok(key)
}

use anyhow::{Context, Result};
use input::{send_inputs, Event, Input};
use sapi_lite::stt::{EventfulContext, Phrase, Recognizer, RuleArena};
use std::sync::Arc;
use std::thread;

use crate::config::Config;

// Only one listener per process should ever be created.
// No other parts of the program should initialize SAPI COM objects.
pub struct Listener {
    // Using Arc to share a Weak pointer with the event handler.
    config: Arc<Config>,
}

impl Drop for Listener {
    fn drop(&mut self) {
        sapi_lite::finalize();
    }
}

impl Listener {
    // Creates a new Listener instance.
    pub fn new(config: Config) -> Result<Self> {
        sapi_lite::initialize()?;

        Ok(Self {
            config: config.into(),
        })
    }

    // Starts listening to commands and blocks the current thread on STDIN.
    pub fn listen_and_block(&self) -> Result<()> {
        let handler = {
            let config = Arc::downgrade(&self.config);
            move |phrase: Phrase| {
                log::trace!("Got phrase: {:#?}", phrase);

                // There should be only one semantic value
                let command = phrase.semantics[0]
                    .value
                    .as_string()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                let config = match config.upgrade() {
                    Some(config) => config,
                    None => {
                        log::error!(
                            "Config has already been dropped, cannot process the command: {}",
                            command
                        );
                        return;
                    }
                };
                let sequence = match config.commands.get(&command) {
                    Some(keys) => keys,
                    None => {
                        log::warn!("Recognition triggered for an unknown command: {}", command);
                        return;
                    }
                };

                log::debug!("Sending {:?} to the active window", sequence);

                for combinations in sequence {
                    let mut down_events = combinations
                        .iter()
                        .map(|key| Input::new(*key, Event::KeyDown))
                        .collect::<Vec<Input>>();

                    let mut up_events = combinations
                        .iter()
                        .map(|key| Input::new(*key, Event::KeyUp))
                        .collect::<Vec<Input>>();

                    if config.timing.hold_duration.is_zero() {
                        down_events.append(&mut up_events);
                        send_inputs(down_events);
                    } else {
                        send_inputs(down_events);
                        thread::sleep(config.timing.hold_duration);
                        send_inputs(up_events);
                    }

                    if !config.timing.sequence_delay.is_zero() {
                        thread::sleep(config.timing.sequence_delay);
                    }
                }
            }
        };

        let recog = Recognizer::new().unwrap();
        let ctx =
            EventfulContext::new(&recog, handler).context("failed to create an EventfulContext")?;

        let arena = RuleArena::new();
        let mut options = Vec::new();
        for command in self.config.commands.keys() {
            options.push(arena.semantic(command.as_str(), arena.text(command)));
        }

        let grammar = ctx
            .grammar_builder()
            .add_rule(arena.choice(options))
            .build()
            .context("failed to build the grammar")?;

        grammar
            .set_enabled(true)
            .context("failed to enable the grammar")?;

        terminal::pause("Listening for voice commands, press any key to exit...");
        Ok(())
    }
}

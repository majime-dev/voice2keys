# voice2keys

This is a simple Windows application that maps a given voice command to a sequence of key presses.

The code might not be particularly rustic, since this is is a toy project made while learning Rust.

A sample configuration file used to augment the gameplay in Hogwarts Legacy is included in the repository root, but this can be used to automate pressing a sequence of keys in other applications.

## Known limitations
* No acoustic echo cancellation (yet!) - use headphones if the audio from your speakers gets picked up as commands
* This is a toy project, so while it might be as good starting point for your own work, support is not guaranteed

## Non-goals
* Supporting platforms other than Windows on x86-64

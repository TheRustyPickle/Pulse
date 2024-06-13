<div align="center"><h1>Pulse</h1></div>
<div align="center">
<a href="https://wakatime.com/badge/github/TheRustyPickle/Pulse"><img src="https://wakatime.com/badge/github/TheRustyPickle/Pulse.svg" alt="wakatime"></a>
</div>
Pulse is a straightforward Discord bot for scheduling messages with some customization options. Built with Rust and the Serenity library, it focuses on ease of use with a simple configuration system.

## Key Features

* Schedule Discord messages in a guild
* Supports various types of messages including file attachments, Polls, and Quizzes
* Robust error handling to ensure the bot does not crash in most cases (unless something fatal occurs)
* Easy to understand configuration system with JSON files, no nested complicated config
* No need for restarts for schedule configuration changes (except for bot config)

## Installation

* Clone the repository
`
git clone https://github.com/TheRustyPickle/Pulse
`
* Update configurations in the `config` folder as required

* Run with Cargo
`
cargo run --release
`

## Configuration

Detailed configuration details can be found on [CONFIG.md file](CONFIG.md). Some example configurations are available in the `config` folder.

## What is a Quiz Message?

This is a niche message system that I built for myself. It's a scheduled message that monitors user messages after being sent. When the first correct reply is found, the bot responds with the message in the config to the user who answered correctly.

## Feedback and Contributions

Have feedback, found a bug, or have a feature request? Feel free to [open an issue](https://github.com/TheRustyPickle/Pulse/issues/new).

## License

Pulse is under the [MIT License](LICENSE).

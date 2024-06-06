<div align="center"><h1>Pulse</h1></div>
<div align="center">
<a href="https://wakatime.com/badge/github/TheRustyPickle/Pulse"><img src="https://wakatime.com/badge/github/TheRustyPickle/Pulse.svg" alt="wakatime"></a>
</div>
Pulse is a simple Discord bot that allows scheduling a few types of messages with some customization. Built with Rust and Serenity with the target of simple configuration system in mind.

<h2>Key Features</h2>

* Schedule Discord messages in a guild
* Supports various types of messages including file attachments, Polls and Quizzes
* Robust error handling to ensure the bot does not crash in most cases (unless something fatal occurs)
* Easy to understand configuration system with JSON files, no nested complicated config
* No need for restarts for schedule configuration changes (except for bot config)

<h2>Installation</h2>

* Clone the repository
`
git clone https://github.com/TheRustyPickle/Rex
`
* Update configurations inn `config` folder as required

* Run with Cargo
`
cargo run --release
`

<h2>Configuration</h2>

The bot looks for the configuration files in `config` folder. All the config files and their structures are shown below.

* Mandatory fields must be within the config.
* Optional fields are not required and the field can be removed completely from the config files.

### bot_config.json

* Global config that are read at the start of the bot only. The bot needs to be restarted upon any changes.

```json
{
    "bot_token": "Discord Bot Token",
    "target_guild": "Guild Name",
    "target_channel": "Channel Name"
}
```

* `bot_token`: (String) A valid discord bot token. Mandatory field
* `target_guild`: (String) The name of the guild to send scheduled message. Mandatory field
* `target_channel`: (String) The name of the channel to send scheduled message. Mandatory field

### completed.json

* Used for saving schedule message IDs that have been completed. Does not require restart upon any changes.
* Usually not for manual editing but can be used to mark a scheduled message as completed by placing the ID here.

```json
{
    "completed": [1, 2, 3]
}
```

* `completed`: (Array of Number) Array of schedule message IDs that have been completed

### schedule.json

* Contains an array that holds all the schedule message data. Does not require restart upon any changes.

```json
[
    {
        "id": 1,
        "message": "Hello, World!",
        "attachments": ["file/image.png", "location/folder_name/video.mp4"],
        "scheduled_at": "2024-01-01 12:00:00 UTC",
        "poll_id": 1,
        "quiz_id": 1,
        "to_pin": true
    }
]
```

* `id`: (Number) Unique ID of the message. Mandatory field
* `message`: (String) Message that is to be sent. Mandatory field
* `scheduled_at`: (String) The time when the message will be sent in UTC. Mandatory field
* `attachments`: (Array of String) Location of files that will be sent as attachments. Optional field
* `poll_id`: (Number) Marks the message as a poll type message and will use the poll data with this ID. Optional field
* `quiz_id`: (Number) Marks the message as a quiz type message and will use the quiz data with this ID. Optional field
* `to_pin`: (Boolean) If true, the message will be pinned. Optional field

### poll.json

* Contains an array that holds all poll data. Does not require restart upon any changes.

```json
{
    "id": 1,
    "question": "Which is your favorite color?",
    "answers": ["Red", "Blue", "Green"]
}
```

* `id`: (Number) Unique ID of the poll. Mandatory field
* `question`: (String) Question of the poll. Mandatory field
* `answers`: (Array of String) Answers to this poll. Must be at least 2 values. Mandatory field

### quiz.json

* Contains an array that holds all quiz data. Does not require restart upon any changes.

```json
{
    "id": 1,
    "answer": "42",
    "end_at": "2024-06-06T12:00:00Z",
    "reply_with": "Congratulations! You've got the right answer!"
}
```

* `id`: (Number) Unique ID of the quiz. Mandatory field
* `answer`: (String) The sentence/word that will be considered as answer to the quiz. Mandatory field
* `reply_with`: (String) The message that will be sent when the user gets the correct answer. Mandatory field
* `end_at`: (String) The time when the quiz will end in UTC. Optional field

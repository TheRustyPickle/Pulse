# Configuration

The bot looks for the configuration files in `config` folder. All the config files, values and their structures are shown below. A few example configurations are already available in the `config` folder.

* Mandatory fields must be within the config files.
* Optional fields are not required and the field can be removed completely from the config files.

### bot_config.json

Global config that are read at the start of the bot only. The bot needs to be restarted upon any changes.

```json
{
    "bot_token": "Discord Bot Token",
    "target_guild": "Guild Name",
    "target_channel": "Channel Name",
    "pin_all": true
}
```

* `bot_token`: (String) A valid discord bot token. Mandatory field
* `target_guild`: (String) The name of the guild to send scheduled message. Also used for quiz answer monitoring. Mandatory field
* `target_channel`: (String) The name of the channel to send scheduled message. Also used for quiz answer monitoring. Mandatory field
* `pin_all`: (Boolean) If true, all scheduled messages will be pinned. Default value is false. Optional field

### completed.json

Used for saving schedule message IDs that have been completed by the bot automatically. Does not require restart upon any changes. Usually not for manual editing but can be used to mark a scheduled message as completed by placing the ID here.

```json
{
    "completed": [1, 2, 3]
}
```

* `completed`: (Array of Number) Array of schedule message IDs that have been completed

### schedule.json

Contains an array that holds all the schedule message data. Does not require restart upon any changes.

```json
[
    {
        "id": 1,
        "message": "Hello, World!",
        "attachments": ["file/image.png", "location/folder_name/video.mp4"],
        "scheduled_at": "2024-01-01 12:00:00 UTC",
        "poll_id": 1,
        "quiz_id": 1,
        "to_pin": true,
        "target_guild": "My Guild Name",
        "target_channel": "My Channel Name"
    }
]
```

* `id`: (Number) Unique ID of the message. Mandatory field
* `message`: (String) Message that is to be sent. Mandatory field
* `scheduled_at`: (String) The time when the message will be sent in UTC. Mandatory field
* `attachments`: (Array of String) Location of files that will be sent as attachments. Optional field
* `poll_id`: (Number) Marks the message as a poll type message and will use the poll data with this ID. Optional field
* `quiz_id`: (Number) Marks the message as a quiz type message and will use the quiz data with this ID. If one is already ongoing, it will be overwritten. Optional field
* `to_pin`: (Boolean) If true, the message will be pinned. Default value is false or taken from `bot_config.json`. Can be used to overwrite the setting on `bot_config.json`. Optional field
* `target_guild`: (String) The name of the guild to send the message. Default value taken from `bot_config.json`. If present, `target_channel` must also be filled up. Optional field
* `target_channel`: (String) The name of the channel to send the message. Default value taken from `bot_config.json`. Optional field

### poll.json

Contains an array that holds all poll data. Does not require restart upon any changes. The ID can be set in a scheduled message to initialize the poll. Same ID can be used in multiple scheduled messages.

```json
{
    "id": 1,
    "question": "Which is your favorite color?",
    "answers": ["Red", "Blue", "Green"],
    "duration_minutes": 2000,
    "multiple_answer": true
}
```

* `id`: (Number) Unique ID of the poll. Mandatory field
* `question`: (String) Question of the poll. Mandatory field
* `answers`: (Array of String) Answers to this poll. Must be at least 2 values. Mandatory field
* `duration_minutes`: (Number) Duration of the poll in minutes. Default value is 1440. Maximum value 10080. Optional field
* `multiple_answer`: (Boolean) If true, multiple answers will be enabled. Default value is false. Optional field

### quiz.json

Contains an array that holds all quiz data. Does not require restart upon any changes. The ID can be set in a scheduled message to initialize the quiz. Same ID can be used in multiple scheduled messages.

```json
{
    "id": 1,
    "answer": "Blue whale",
    "reply_with": "Congratulations! You've got the right answer!",
    "end_at": "2024-06-06T12:00:00Z",
    "monitor_guild": "My Guild Name",
    "monitor_channel": "My Channel Name"
}
```

* `id`: (Number) Unique ID of the quiz. Mandatory field
* `answer`: (String) The sentence or word that will be considered as answer to the quiz. Mandatory field
* `reply_with`: (String) The message that will be sent to the user who gives the correct answer. Mandatory field
* `end_at`: (String) The time when the quiz will end in UTC. Default value is no end time. Replies after this will not be checked. Optional field
* `monitor_guild`: (String) The name of the guild that will be monitored for the quiz answer. Default value taken from `bot_config.json`. If present, `monitor_channel` must also be filled up. Optional field
* `monitor_channel`: (String) The name of the channel that will be monitored for the quiz answer. Default value taken from `bot_config.json`. Optional field

## Further questions

If something is still confusing, need more info or want to request for a specific configuration, feel free to [open an issue](https://github.com/TheRustyPickle/Pulse/issues/new).

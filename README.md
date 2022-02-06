# pushover_cli

A command line app to send notifications using the [Pushover] [API], written in
Rust.

## Testing

Send a notification by running

    cargo run -- --user <user> --token <token> --message "test message"

[Pushover]: https://pushover.net
[API]: https://pushover.net/api

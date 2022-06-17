# Bot_OW

Stupid Discord-Bot, that adds `_ow` to new members nicknames.
Used in [Cicero's University](https://discord.gg/Fj7yUqF)


## Install
### Build-Dependencies
* [rust](https://github.com/rust-lang/rust)

### Run
```bash
# 1. Clone the repo
git clone git@github.com:SimonZehetner/bot_ow.git && cd bot_ow
# 2. Compile
cargo build --release
# 4. Provide the discord token and run the Bot
target/release/bot_ow "<your_token_goes_here>" &
# Alternatively, you can specify the `DISCORD_TOKEN` environment variable
```
## Usage
When a new member joins the server, `_ow` is appended to their nickname. The users can change their nickname back.

### Commands
#### `!overwatching`
Appends `_ow` to all members of the server (if not already ends with `_ow`). Can be used for initial "setup" of the server.

> Only users with the `Manage Nicknames`-permission are allowed to execute this command!

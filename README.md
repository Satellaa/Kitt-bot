# Smol Lilac [<img src="https://img.shields.io/badge/invite%20to-discord-brightgreen?style=for-the-badge" alt="Invite to Discord" align="right" />](https://discord.com/api/oauth2/authorize?client_id=1082275634757242890&permissions=277025474560&scope=bot%20applications.commands)

A Discord bot for looking up card prices in Yu-Gi-Oh! Official Card Game.

See the [docs](docs/commands) for details on how to use the commands.

All card information, such as names, database IDs, etc., is sourced from [YAML Yugi](https://github.com/DawnbrandBots/yaml-yugi).

## Discord permissions

Please ensure that you use an [invite link](https://discord.com/api/oauth2/authorize?client_id=1082275634757242890&permissions=277025474560&scope=bot%20applications.commands) that automatically grants the permissions listed below.

- Create commands in a server
- Send Messages
- Send Messages in Threads
- Embed Links: Smol Lilac uses a rich embed in Discord to show card prices.
- Read Message History: Smol Lilac replies to messages that request card prices search.

Deny Smol Lilac the permission to View Channel in any channel where you do not want it to be used.
Otherwise, for Smol Lilac to function properly, any channel in which it is accessible **must** provide it all of the aforementioned permissions.

## Contributing

Smol Lilac is written in [Rust](https://www.rust-lang.org/)
and runs on [Shuttle](https://www.shuttle.rs/).
It interfaces with Discord via the framework [poise](https://github.com/serenity-rs/poise), which is built on [serenity-rs](https://github.com/serenity-rs/serenity).
 
Please use tabs rather than spaces.
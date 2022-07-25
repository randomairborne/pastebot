# pastebot

A Discord bot for serving files.

## Setup

The bot can be run if you have docker compose with the below command, setting PASTEBIN to, say, `paste.valk.sh`, and DISCORD_TOKEN to your Discord bot token.

```bash
wget https://raw.githubusercontent.com/randomairborne/pastebot/main/compose.yml
PASTEBIN=paste.valk.sh DISCORD_TOKEN=token.yqisa docker-compose up -d
```

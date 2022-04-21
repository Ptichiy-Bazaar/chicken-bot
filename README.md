# 🐦 Chicken Bot

Utility bot for [Птичий базар](https://discord.gg/jdVWPYjx3q) Discord server that supports features as

- [Support tickets](#💁‍♀️-support-ticket)
- [Emoji role manager](#😁-emoji-role)
- [Design news aggregator](#news-aggregation)

## ⚡ Quickstart

Copy [`.env.example`](/.env.example) file to `.env` and fill it with your env variables.

```dockerfile
DISCORD_TOKEN="put your token here"
REDIS_URL="redis connection url"

# Declares the level of logging to use.
RUST_LOG=debug
```

Run a bot

```sh
cargo run --release
```

## 🎧 Support ticket

<!-- TODO: Описать процесс открытия тикета, и последующие шаги (назначить куратора, архивация) -->

![embed with 3 interactions buttons: complain, proposal, verification](https://user-images.githubusercontent.com/26527529/161784545-9587028e-e913-414b-8cbf-6dca990e9ae7.png)

## 😁 Emoji role

<!-- TODO: Описать как конфигурировать эту хрень -->

![embed with emoji](https://user-images.githubusercontent.com/26527529/161785658-dac87021-aa22-41fa-849f-8cb10c4d3903.png)

## News Aggregation

<!-- TODO: Описать все сайты который умеет парсить бот, а также канал верификации новостей -->

![webhook with news from skillbox.ru](https://user-images.githubusercontent.com/26527529/161785399-cab0414b-4ce9-4fc4-85bf-32811781202e.png)

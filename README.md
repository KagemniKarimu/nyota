![logo](/logo_with_text.png)

pronounced as /ˈɲɔ.ta/ (nyoh - TAH) in Swahili, means star.
It is a fast, modular, extensible crypto-native AI agent platform written in Rust. It is designed to do a wide range of tasks.


## Features
- [x] Modular - use parts of the program as needed (i.e. no color, no sound, no splash)
- [x] Extensible - can be extended with new features through plugins
- [x] Fast - written in Rust with performance in mind
- [x] Crypto-native - blockchain interactions are first class citizens
- [x] Agentic - can act on its own to interpret and achieve goals
- [x] Soundful - generates pleasing and interesting soundscape as part of its tui
- [x] Colorful - generates colorful visualizations for the modern terminal
- [x] Fun - Need we say more?

## Run it

1. Clone the repository and change into the directory
```bash
git clone https://github.com/KagemniKarimu/nyota.git
cd nyota
```

2. Copy .env.example to .env
```bash
cp .env.example .env
//nano, helix, vim, gedit .env- however you like :)
```

3. Add your API keys to the .env file


NOTE: Your API keys are sensitive information. Do not show this file to anyone.
```env

ANTHROPIC_API_KEY=
OPENAI_API_KEY=
OPENROUTER_API_KEY=
GROK_API_KEY=
```

4. Select your default model in the .env file
```env

NYOTA_DEFAULT_AI_MODEL=claude-3-5-sonnet-20241022
```

If using openrouter or ollama, preface the model with `openrouter/` or `ollama/` respectively.

5. Run nyota ! ✴️

```bash
cargo run
```


---
## Contributing

`nyota` is an open-source collaboratve effort! All contributions are welcome.
If you would like to contribute to this project, please read the [contribution rules](/notes/contribution-rules.md) file for more information.

Once you're ready to dig in -a good place to start is to read the documentation
```bash
cargo doc --open --no-deps
```

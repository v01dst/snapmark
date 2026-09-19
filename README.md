# snapmark

**A local-first knowledge pocket for your terminal.**

Save bookmarks, commands, snippets, docs, and useful text without accounts, cloud sync, or API keys.

## Features

- Fast native Rust CLI
- Local JSON storage
- Full-text search across names, values, and tags
- Tags for organizing entries
- Rename and delete
- Script-friendly output
- No network dependency

## Commands

```bash
snapmark add rust https://rust-lang.org --tags lang,reference
snapmark add deploy "cargo build --release" --tags command,rust

snapmark list
snapmark list --tag rust
snapmark search cargo
snapmark get deploy

snapmark rename deploy release-build
snapmark tag release-build command rust
snapmark rm release-build
snapmark clear
```

## Design

snapmark is intentionally local-first: your data lives on your machine as readable JSON. There is no account system and no server.

## Roadmap

- [ ] Markdown snippets with files
- [ ] Import/export JSON and Markdown
- [ ] Fuzzy ranking
- [ ] Shell completions
- [ ] Interactive TUI
- [ ] Optional encryption

## License

MIT

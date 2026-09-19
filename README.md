# snapmark

**A local-first developer knowledge vault — CLI + local web UI, no account required.**

Store bookmarks, commands, snippets, docs, notes, and useful text on your machine. The same JSON store powers the terminal and the browser UI.

## Why snapmark

- **Local-first** — your data stays on your machine.
- **Native CLI** — fast, scriptable Rust binary.
- **Web UI** — launch a local dashboard with one command.
- **Search** — search names, content, and tags.
- **Portable** — JSON import/export.
- **Safe writes** — atomic file replacement.
- **No cloud dependency** — no API keys, accounts, or network required for core storage.

## Install

### From source

```bash
cargo install --path .
```

### Use

```bash
snapmark add rust https://rust-lang.org --tags lang,reference
snapmark add deploy "cargo build --release" --tags command,rust
snapmark add api "https://example.com/docs" --tags docs,web

snapmark list
snapmark list --tag rust
snapmark search cargo
snapmark get deploy
snapmark stats
snapmark path
```

## Web dashboard

```bash
snapmark serve
```

Then open **http://127.0.0.1:8787**.

The UI talks directly to the local process. Nothing is uploaded.

## Data portability

```bash
snapmark export backup.json
snapmark import backup.json
snapmark import backup.json --replace
```

## Destructive operations

```bash
snapmark clear --yes
```

## Architecture

```text
                 ┌────────────────────┐
                 │     snapmark CLI   │
                 └─────────┬──────────┘
                           │
                 ┌─────────▼──────────┐
                 │   Rust core/store  │
                 └─────────┬──────────┘
                           │
              ┌────────────┴────────────┐
              │                         │
        JSON on disk              Local HTTP API
                                      │
                               HTML/CSS/JS UI
```

Rust handles the core, storage, CLI, and local server. The browser layer is plain HTML/CSS/JavaScript so it stays tiny and dependency-free.

## Development

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Roadmap

- [x] CLI CRUD
- [x] Tags
- [x] Search
- [x] JSON import/export
- [x] Atomic writes
- [x] Local web dashboard
- [ ] Fuzzy ranking
- [ ] Shell completions
- [ ] Rich snippet types
- [ ] TUI
- [ ] Optional encryption
- [ ] Plugins/integrations

## License

MIT

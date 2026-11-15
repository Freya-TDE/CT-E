# CT-Editor – Cargo TOML Editor

**Deutsch:** Visueller Editor für `Cargo.toml`-Dateien, gebaut mit
[RUF/RNCL](https://github.com/Dragon-InterActive/RUF) – GTK 4 auf Linux, Windows und macOS. Eigenständig
nutzbar (mit Menü „Datei“) oder als Teil des RAD-Tools
[Freya TDE](https://github.com/Dragon-InterActive/Freya-TDE), das ihn als Submodul einbindet.

**English:** A visual editor for `Cargo.toml` files, built with
[RUF/RNCL](https://github.com/Dragon-InterActive/RUF) – GTK 4 on Linux, Windows and macOS. Usable on its own
(with a "File" menu) or as part of the RAD tool [Freya TDE](https://github.com/Dragon-InterActive/Freya-TDE),
which includes it as a submodule.

> **Mit Claude Code gebaut / Built with Claude Code** – Der CT-Editor wurde vollständig mit
> [Claude Code](https://claude.com/claude-code) (Modell Claude Opus 5.5) umgesetzt. / The CT-Editor was built
> entirely with [Claude Code](https://claude.com/claude-code) (model Claude Opus 5.5).

![CT-Editor](docs/images/dependencies-de.png)

## Was er kann / What it does

| Deutsch | English |
|---|---|
| **Quelltext** mit Syntax-Hervorhebung und Live-Prüfung | **source** with syntax highlighting and live validation |
| **Abhängigkeiten:** Suche auf crates.io (ab 3 Zeichen), Features per Häkchen, vorbelegt mit den Default-Features des Crates | **dependencies:** crates.io search (3+ characters), features as check boxes, preset with the crate's default features |
| **Paket**, **Features**, **Erweitert** (jeder weitere Abschnitt: `[lib]`, `[[bin]]`, `[profile.*]`, `[workspace]` …) | **package**, **features**, **advanced** (every further section: `[lib]`, `[[bin]]`, `[profile.*]`, `[workspace]` …) |
| formatierungserhaltend (Kommentare bleiben), Prüfung durch `cargo metadata` | format-preserving (comments stay), validated by `cargo metadata` |
| Deutsch und Englisch, weitere Sprachen als TOML-Datei; Darstellung System/Hell/Dunkel | German and English, further languages as a TOML file; appearance system/light/dark |

## Aufruf / Usage

```bash
ct-editor --path /pfad/zum/projekt               # eigenständig / standalone
ct-editor --embedded --lang de --theme dark --path /pfad/zum/projekt   # so ruft Freya TDE ihn auf / how Freya TDE starts it
ct-editor --theme light                          # Darstellung / appearance: system (Standard/default), light, dark
cargo test                                       # Tests (Linux ohne Bildschirm / without a display: xvfb-run -a cargo test)
```

Fertige Pakete für Linux, Windows und macOS gibt es auf der
[Release-Seite](https://github.com/Dragon-InterActive/CT-E/releases). / Ready-made packages for Linux, Windows
and macOS are on the [release page](https://github.com/Dragon-InterActive/CT-E/releases).

## Dokumentation / Documentation

- 🇩🇪 [Handbuch](docs/de/README.md)
- 🇬🇧 [Manual](docs/en/README.md)
- KI / AI: [`AGENTS.md`](AGENTS.md), [`.agents/skills/ct-editor`](.agents/skills/ct-editor/SKILL.md)

## Lizenz / License

Nach Wahl lizenziert unter / Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

**Deutsch:** Beiträge werden, sofern nicht ausdrücklich anders angegeben, unter denselben beiden Lizenzen
eingebracht.

**English:** Unless explicitly stated otherwise, contributions are dual-licensed as above.

© Dragon Software

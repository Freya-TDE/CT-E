---
name: ct-editor
description: Extend the CT-Editor (Cargo TOML editor of Freya TDE) or look for bugs in it – new tabs, package fields, manifest functions, translations, crates.io connection, GUI tests and screenshots. Use when working in the CT-Editor folder or when Freya TDE is to edit Cargo.toml files.
---

# Extending the CT-Editor

Read `AGENTS.md` first (rules), then the matching recipe. UI basics: RUF skill `ruf-app`.

## What goes where?

| Task | File | Test |
|---|---|---|
| Read/change the manifest (new sections, notations) | `src/manifest.rs` | unit test in the same module |
| Feature logic | `src/features.rs` | unit test |
| crates.io / other registry | `src/crates_io.rs` (`CrateSource`) | unit test + `#[ignore]` live test |
| Text in the user interface | `locales/de.toml` **and** `locales/en.toml` (`ruf::i18n`) | `cargo test --lib i18n` |
| Widgets, layout, operation | `src/ui/<tab>.rs` | `tests/gui.rs` |
| Start options | `src/cli.rs` | unit test |

## Recipe: new `[package]` field

1. Entry in `PACKAGE_FIELDS` (`src/manifest.rs`) with a matching `FieldKind`.
2. Tooltip `package.field.<key>` in both `locales/*.toml`.
3. Done – form, loading and saving are generic (`package_tab.rs`).
   The test `package_field_tooltips_exist` checks the translation.

## Recipe: new section template (tab "Advanced")

Extend `SECTION_TEMPLATES` in `src/manifest.rs`: `("key", "[toml]\n…")`.
The test `sections` checks that every template yields valid TOML.

## Recipe: new tab

1. `src/ui/new_tab.rs` modelled on `features_tab.rs`: `pub struct NewTab { pub page: RnclPanel, … }`
   with `build(page)`, `layout(w, h)`, `refresh(&State)` and `pub fn connect(shared: &Rc<Shared>)`.
2. In `ui/mod.rs`: `pub mod`, field in `Shared`, `NewTab::build(&tabs.add_tab(&tr!("tab.new")))`,
   `new_tab::connect(&shared)`, `Origin::New`, entry in `Shared::refresh`.
3. Add the variant to `cli::Tab` (order = tab index).
4. Texts in both language files; add the file to the list in the i18n test `every_used_key_exists`.
5. GUI test in `tests/gui.rs` (function **and** entry in the `tests` list).

Pattern in `connect`:

```rust
tab.button.on_clicked(weak0(shared, |s| {
    s.modify(Origin::New, |m| { /* change the manifest */ });
    s.quietly(|state| s.new.refresh(state));          // update the own tab
    s.set_status(StatusKind::Success, &tr!("new.done"));
}));
tab.field.on_text_changed(weak1(shared, |s, text: String| { /* … */ }));
```

## Recipe: background work (network, processes)

Model `deps_tab.rs` (`start_search` / `on_search_reply`): increase the generation counter,
`std::thread::spawn`, send the result back via `GuiSender` (created in `connect`, stored in a
`OnceCell` in the tab), compare the generation in the handler.

## Checking

```bash
cargo clippy --all-targets && cargo test
./target/debug/ct-editor --offline --lang en --path /tmp/demo --tab <tab> --screenshot /tmp/ct.png
xvfb-run -a cargo run --example remote_control -- /tmp/demo serde derive /tmp/d.png
```

Look at the image: overlaps, cut-off texts, empty areas. For layout problems check the tab's
`layout(w, h)` function (absolute positions, minimum sizes via `clamp`).

## Pitfalls

- Signals also fire on `set_text`/`set_items` → endless loops or overwritten input if a
  handler does not run through `weak1` or the tab is not filled `quietly`.
- `RnclComboBox`: set the text with `set_current_text`, not `set_text`.
- While the source has a syntax error, `modify` ignores all changes (on purpose).
- The feature order in the manifest is kept (`apply_selection`) – align tests with that.

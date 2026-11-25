//! Mehrsprachigkeit (i18n).
//!
//! Übersetzungen liegen als TOML-Dateien unter `locales/<sprache>.toml` und
//! werden in das Programm eingebettet; die Technik ist `ruf::i18n` (RUF ≥ 0.4). Freya TDE kann zusätzlich einen Ordner
//! mit weiteren `*.toml`-Dateien übergeben (`--locales`), um Sprachen ohne
//! Neukompilieren nachzurüsten oder Texte zu überschreiben.
//!
//! Schlüssel sind verschachtelte Tabellen in Punktschreibweise
//! (`[deps] search = "…"` → `deps.search`). Platzhalter stehen in
//! geschweiften Klammern: `"Zeile {line}"`.
//!
//! ```
//! use ct_editor::{i18n, tr};
//!
//! i18n::set_language("de");
//! assert_eq!(tr!("app.save"), "Speichern");
//! assert_eq!(tr!("status.parse_error", line = 3, message = "x"), "Syntaxfehler in Zeile 3: x");
//! ```

use std::path::Path;
use std::sync::Once;

pub use ruf::i18n::FALLBACK;

const EMBEDDED: &[(&str, &str)] = &[
    ("de", include_str!("../locales/de.toml")),
    ("en", include_str!("../locales/en.toml")),
];

/// Meldet die eingebetteten Texte einmal bei `ruf::i18n` an (die Technik
/// liegt seit RUF 0.4 dort, für alle Programme auf RUF).
fn ensure() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| ruf::i18n::register(EMBEDDED));
}

/// Stellt die Sprache ein und liefert die tatsächlich verwendete.
///
/// Unbekannte Sprachen fallen auf die Hauptsprache (`de-AT` → `de`) und
/// zuletzt auf [`FALLBACK`] zurück.
pub fn set_language(language: &str) -> String {
    ensure();
    ruf::i18n::set_language(language)
}

/// Die aktuell eingestellte Sprache.
pub fn language() -> String {
    ensure();
    ruf::i18n::language()
}

/// Der Name der Sprache `language` in ihr selbst (`de` → „Deutsch“).
pub fn name_of(language: &str) -> String {
    ensure();
    ruf::i18n::name_of(language)
}

/// Alle verfügbaren Sprachen (sortiert).
pub fn available() -> Vec<String> {
    ensure();
    ruf::i18n::available()
}

/// Lädt alle `*.toml`-Dateien aus `dir` als zusätzliche Sprachen.
pub fn load_dir(dir: &Path) -> Result<Vec<String>, String> {
    ensure();
    ruf::i18n::load_dir(dir)
}

/// Übersetzt `key` in die aktuelle Sprache (Rückfall Englisch, zuletzt der
/// Schlüssel selbst).
pub fn translate(key: &str) -> String {
    ensure();
    ruf::i18n::translate(key)
}

/// Wie [`translate`], ersetzt aber `{name}`-Platzhalter.
pub fn translate_with(key: &str, args: &[(&str, String)]) -> String {
    ensure();
    ruf::i18n::translate_with(key, args)
}

/// Alle Schlüssel einer Sprache (für Vollständigkeitstests).
pub fn keys(language: &str) -> Vec<String> {
    ensure();
    ruf::i18n::keys(language)
}

/// Übersetzt einen Schlüssel: `tr!("app.save")` oder mit Platzhaltern
/// `tr!("status.saved", path = p.display())`.
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::i18n::translate($key)
    };
    ($key:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::i18n::translate_with($key, &[$((stringify!($name), $value.to_string())),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_are_complete() {
        let reference = keys(FALLBACK);
        assert!(!reference.is_empty());
        for lang in available() {
            assert_eq!(keys(&lang), reference, "Sprache {lang} unvollständig");
        }
    }

    #[test]
    fn every_used_key_exists() {
        // Alle tr!("…")-Aufrufe im Quelltext müssen übersetzt sein.
        let sources = [
            include_str!("main.rs"),
            include_str!("ui/mod.rs"),
            include_str!("ui/source_tab.rs"),
            include_str!("ui/deps_tab.rs"),
            include_str!("ui/package_tab.rs"),
            include_str!("ui/features_tab.rs"),
            include_str!("ui/advanced_tab.rs"),
            include_str!("crates_io.rs"),
            include_str!("cargo_cli.rs"),
        ];
        let known = keys(FALLBACK);
        for source in sources {
            for part in source.split("tr!(\"").skip(1) {
                let key = part.split('"').next().unwrap();
                assert!(known.iter().any(|k| k == key), "Schlüssel fehlt: {key}");
            }
        }
    }

    #[test]
    fn package_field_tooltips_exist() {
        let known = keys(FALLBACK);
        for field in crate::manifest::PACKAGE_FIELDS {
            let key = format!("package.field.{}", field.key);
            assert!(known.contains(&key), "Schlüssel fehlt: {key}");
        }
        for key in ["deps.added", "deps.updated", "source.error"] {
            assert!(known.iter().any(|k| k == key), "Schlüssel fehlt: {key}");
        }
    }

    #[test]
    fn fallback_to_the_key() {
        assert_eq!(translate_with("x.unknown", &[]), "x.unknown");
    }
}

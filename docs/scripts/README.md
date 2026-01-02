# Rust Documentation Integration

## Status: Nightly Required

Der JSON-Export von `rustdoc` ist **noch nicht im stabilen Release** verfügbar. Aktuell (Rust 1.91.1) ist die `nightly` Toolchain erforderlich.

**Fehlermeldung bei stabil:**
```
error: the -Z unstable-options flag must be passed to enable --output-format
```

**GitHub Issue:** https://github.com/rust-lang/rust/issues/76578

## Dokumentationsumfang

### Aktuell: Nur öffentliche API

Standardmäßig exportiert `rustdoc` nur **öffentliche** Items (`pub`). Das ist für eine öffentliche Dokumentationsseite sinnvoll.

**JSON-Feld:** `includes_private: false`

### Option: Interne Dokumentation

Falls interne Dokumentation gewünscht ist, kann `--document-private-items` hinzugefügt werden:

```bash
cargo +nightly rustdoc -p engine -- --output-format json -Z unstable-options --document-private-items
```

**Empfehlung:** Für die öffentliche Dokumentationsseite sollten wir bei öffentlichen Items bleiben. Interne Dokumentation kann separat generiert werden (z.B. für Entwickler-Dokumentation).

## Strukturierung in Starlight

### Hierarchische Modulstruktur

Der Generator erstellt automatisch eine **hierarchische Struktur**:

```
reference/rust/
├── index.mdx          # Root crate (engine)
├── module1/
│   └── index.mdx      # module1
├── module2/
│   ├── index.mdx      # module2
│   └── submodule/
│       └── index.mdx  # module2::submodule
```

### Vorteile dieser Struktur:

1. **Automatische Sidebar-Navigation:** Starlight scannt das Verzeichnis automatisch und erstellt die Sidebar
2. **Kleine, fokussierte Seiten:** Jedes Modul bekommt seine eigene Seite
3. **Natürliche Hierarchie:** Die Verzeichnisstruktur spiegelt die Rust-Modulstruktur wider
4. **Bessere Performance:** Kleinere Seiten laden schneller

### Beispiel-Struktur für `engine` Crate:

```
reference/rust/
├── index.mdx                    # engine (Übersicht)
├── rpc/
│   └── index.mdx                # engine::rpc
├── core/
│   └── index.mdx                # engine::core
├── orchestrator/
│   └── index.mdx                # engine::orchestrator
└── hal/
    └── index.mdx                # engine::hal
```

## Zukünftige Verbesserungen

1. **Stable Release:** Sobald `--output-format json` stabil ist, können wir `+nightly` entfernen
2. **Erweiterte Typ-Formatierung:** Bessere Darstellung von komplexen Typen (Generics, Lifetimes, etc.)
3. **Cross-References:** Links zwischen verwandten Items
4. **Code-Beispiele:** Extraktion von Beispielen aus Doc-Tests


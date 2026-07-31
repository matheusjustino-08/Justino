# Justino Project Conventions and Code Guidelines

This document outlines the coding standards, safety rules, and internationalization governance for the Justino programming language codebase (`justino_core`, `justino_ui`, `justino_stdlib`, and tools).

---

## 1. Rust Code Quality & Edition Standards

- **Rust Edition**: Rust Edition 2021/2024.
- **Strict Warning Governance**: All production crates must pass `cargo check` and `cargo test` with **zero warnings** and **zero errors**.
- **Clippy Compliance**: Code must adhere to `cargo clippy -- -D warnings`.

---

## 2. Industrial Safety Rules

1. **PROHIBITED: `unsafe` Keyword**
   - The use of `unsafe` blocks is strictly forbidden across all parser, compiler, VM, and layout modules.
   - Shared ownership and interior mutability must be handled safely via `Rc<T>`, `RefCell<T>`, or arena reference-counting mechanisms.

2. **PROHIBITED: `panic!`, `unwrap()`, and `expect()` in Production Code**
   - Production runtime, compiler, lexer, and parser modules must NEVER invoke `panic!`, `.unwrap()`, or `.expect()`.
   - All fallible operations (file reading, numeric conversions, vector index access, syntax parsing, runtime instruction decoding) must return explicit `Result<T, JustinoError>` or `Result<T, UiError>` types.
   - `unwrap()` and `expect()` are permitted ONLY inside unit/integration test files (`tests/*.rs`).

3. **Span Tracking & Preservation**
   - All AST nodes, tokens, and bytecode instructions must retain source position metadata using `Span { file_id, start, end, line, column }`.
   - Error messages produced by the compiler or VM must display exact line and column numbers.

---

## 3. Two-Layer Internationalization (i18n) Governance

### Layer 1: Developer Layer (Compiler & LSP Diagnostics)
- Compiler error messages must NOT be hardcoded as ad-hoc strings in deep parser logic.
- Diagnostic errors must reference i18n message keys mapped through translation catalogs (`pt-BR`, `en-US`, `es-ES`, `zh-CN`).
- When introducing a new diagnostic error variant in `JustinoError`:
  1. Add the enum variant with associated `Span`.
  2. Register the default English diagnostic template and localized translation string.

### Layer 2: Application Layer (UI & Runtime Engine)
- The runtime UI engine (`justino_ui`) and stdlib must parse BCP 47 locale tags (`pt-BR`, `ar-SA`, `en-US`, etc.).
- When `locale.is_rtl()` is true (e.g. `ar-SA` or `he-IL`), the Bidi layout engine (`BidiEngine`) automatically mirrors:
  - Flex direction (`Row` <-> `RowReverse`)
  - Horizontal margins (`margin-left` <-> `margin-right`)
  - Horizontal paddings (`padding-left` <-> `padding-right`)
  - Text alignment (`FlexStart` <-> `FlexEnd`)

---

## 4. File Extensions & Naming Conventions

- **Source Code**: Official extension is `.jucode` (e.g., `app_demo.jucode`, `main.jucode`).
- **Stylesheets**: Official extension is `.css` (e.g., `styles.css`).
- **Rust Modules**: Lowercase snake_case (`box_model.rs`, `font_shaper.rs`).
- **Rust Structs / Enums**: PascalCase (`CompiledFunction`, `JustinoError`, `BidiEngine`).

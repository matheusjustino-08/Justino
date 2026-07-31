# Justino Language Architecture Specification (`.jucode`)

## 1. Overview & Universal Design Principles

**Justino** is a high-performance, memory-safe, embeddable programming language designed for modern software development, graphical user interface (UI) rendering, and concurrent systems programming.

### Key Architectural Pillars
- **Official Extension**: `.jucode` for source files, `.css` for declarative styling stylesheets.
- **Universal English Syntax**: Language keywords (`fn`, `let`, `mut`, `struct`, `async`, `await`, `return`, `if`, `else`, `match`, `spawn`, `import`, `export`, `try`, `catch`), built-in types, and stdlib APIs strictly use **English** as the international engineering standard.
- **Two-Layer Internationalization (i18n)**:
  - **Dev Layer (Compiler & LSP Diagnostics)**: Diagnostic messages, parser error reports, and compiler warnings are backed by localization catalogs (`pt-BR`, `en-US`, `es-ES`, `zh-CN`) to present friendly messages in the developer's native language.
  - **App Layer (UI & Runtime)**: Built-in support for BCP 47 locales, Unicode UTF-8 font shaping, and automatic **Bi-Directional (Bidi)** layout mirroring for Right-to-Left (RTL) scripts (such as Arabic and Hebrew).
- **Absolute Memory Safety in Rust**: Built using Rust Edition 2021/2024 with **Zero `unsafe`**, **Zero `panic!`**, **Zero `unwrap()`**, and **Zero `expect()`** in production code. All fallible operations return `Result<T, JustinoError>`.

---

## 2. Core Language Pipeline (`justino_core`)

The compilation and execution engine follows a register-based virtual machine pipeline:

```text
  +--------------------+      +--------------------+      +--------------------+      +--------------------+
  |  .jucode Source    | ---> |  UTF-8 Lexer       | ---> |  Pratt Parser      | ---> |  Register Bytecode |
  |  (Unicode UTF-8)   |      |  (Token Stream)    |      |  (AST Hierarchy)   |      |  Compiler          |
  +--------------------+      +--------------------+      +--------------------+      +--------------------+
                                                                                            |
                                                                                            v
                                                                                  +--------------------+
                                                                                  |  Register-Based VM |
                                                                                  |  (CallFrames & GC) |
                                                                                  +--------------------+
```

### Component Breakdown
1. **Span & Diagnostic Location (`span.rs` & `error.rs`)**:
   - Every token, AST node, and bytecode instruction tracks a `Span { file_id, start, end, line, column }`.
   - Unified `JustinoError` enum handles lexical, syntactic, compilation, and runtime errors with exact source coordinates.
2. **UTF-8 Scanner (`lexer/`)**:
   - Scans UTF-8 streams supporting Unicode variable identifiers (e.g. `coração`), emojis, comments (`//`, `/* */`), numeric literals (`i64`, `f64`), and string interpolation (`"Hello, ${name}!"`).
3. **Pratt Parser & AST (`parser/`)**:
   - Parses statements (`Let`, `Assignment`, `FunctionDef`, `StructDef`, `If`, `While`, `Return`) and expressions (`Binary`, `Unary`, `Call`, `StructInit`, `MemberAccess`, `Spawn`, `Await`, `InterpolatedString`).
   - Algorithmic binding power prevents recursion stack overflow and handles operator precedence gracefully.
4. **Register Bytecode Compiler (`compiler/`)**:
   - Translates AST nodes into register instructions (`LoadConst`, `Move`, `Add`, `Sub`, `Call`, `Return`, `Jump`, `JumpIfFalse`, `NewStruct`, `SetField`, `GetField`, `ConcatStrings`).
   - Manages lexical register allocation per `CallFrame` (up to 256 registers per frame).
5. **Virtual Machine & GC Arena (`vm/`)**:
   - Register-based Fetch-Decode-Execute dispatch loop.
   - Values represented by the runtime `Value` enum (`Int`, `Float`, `Bool`, `String`, `Null`, `Object`, `Function`, `StructInstance`).
   - Memory managed safely via `GcArena` and `Rc<RefCell<...>>` without `unsafe` blocks.

---

## 3. Graphical UI Engine (`justino_ui`)

`justino_ui` provides a GPU-accelerated, declarative UI rendering system:

```text
  +--------------------+      +--------------------+      +--------------------+      +--------------------+
  |  .css Stylesheet   | ---> |  CSS3 Parser       | ---> |  Flexbox Engine    | ---> |  Render Context    |
  |  (Declarative CSS) |      |  (Cascade & Spec)  |      |  (Bidi LTR/RTL)    |      |  (GPU Primitives)  |
  +--------------------+      +--------------------+      +--------------------+      +--------------------+
```

### Component Breakdown
1. **CSS3 Parser & Cascade Engine (`css/`)**:
   - Parses CSS selectors (Tag, Class `.class`, ID `#id`, Pseudo-class `:hover`, `:active`, `:focus`, `:lang(...)`).
   - Computes selector specificity tuple `(id, class, tag)` and rule cascade.
   - Parses length units (`px`, `rem`, `em`, `%`, `vh`, `vw`) and Hex/RGBA colors.
2. **Box Model & Flexbox Layout (`layout/`)**:
   - `UiNode` tree representation (`Element` vs `Text`).
   - Box Model geometry (`content`, `padding`, `border`, `margin`).
   - Solves Flexbox main & cross axes, `gap`, `flex-grow`, `justify-content`, and `align-items`.
3. **Bi-Directional i18n System (`i18n/`)**:
   - Parses BCP 47 locale tags (`pt-BR`, `en-US`, `ar-SA`, `he-IL`).
   - `BidiEngine` automatically mirrors horizontal flex direction (`Row` -> `RowReverse`), horizontal margins/paddings, and text alignments for RTL script locales.
4. **GPU Renderer & Widgets (`render/` & `widget/`)**:
   - `RenderContext` command buffer queue (`FillRect`, `DrawBorder`, `DrawText`, `DrawShadow`).
   - `Painter` traverses `UiNode` tree to generate GPU draw calls.
   - `Window`, `Container`, `Text`, `Button`, and `Input` widget component library.

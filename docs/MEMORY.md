# Justino Architectural Memory & Decision Records (ADRs)

Este documento registra as decisões de arquitetura de software, o estado atual dos subsistemas e o registro de memória do projeto **Justino** (`.jucode`).

---

## 1. Arquitetura da Máquina Virtual de Registradores (`justino_core`)

### Registradores e CallFrames
- **Instruções Baseadas em Registradores**: A VM opera sobre registradores indexados (`u8`, R0 a R255) em cada `CallFrame`.
- **Zero Stack Push/Pop**: Operações aritméticas e relacionais leem diretamente dos registradores de entrada e escrevem no registrador de destino.
- **Tabela de Opcodes**:
  - `LoadConst { dst, const_idx }`, `Move { dst, src }`, `LoadNull { dst }`, `LoadBool { dst, val }`.
  - `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Neg`, `Not`.
  - `Equal`, `NotEqual`, `LessThan`, `GreaterThan`, `LessEqual`, `GreaterEqual`.
  - `Jump { offset }`, `JumpIfFalse { condition, offset }`.
  - `Call { dst, func_reg, arg_start, arg_count }`, `Return { src }`, `Spawn { func_reg }`.
  - `NewStruct`, `SetField`, `GetField`, `ConcatStrings`.

---

## 2. Motor Gráfico de UI & Bidi Layout (`justino_ui`)

### Componentes Principais
- **CSS3 Parser & Specificity**: Parsing sem panics com especificidade calculada por tupla `(id, class, tag)`.
- **Flexbox Spatial Layout**: Resolução dos eixos principal e cruzado, margens, paddings, borders e `flex-grow`.
- **Bidi Engine**: Espelhamento automático para locales RTL (`ar-SA`, `he-IL`) invertendo `flex-direction`, margens/paddings horizontais e alinhamento de texto.
- **GPU Native Desktop App Windowing Engine**: Aceleração gráfica via GPU na janela nativa da plataforma (WebView2 no Windows / WebKitGTK no Linux / WKWebView no macOS).

---

## 3. Biblioteca Padrão NAtiva ("Batteries Included") (`justino_stdlib`)

A biblioteca padrão nativa do Justino expõe 7 módulos principais registrados diretamente na VM:

| Módulo | API Principal | Função |
| :--- | :--- | :--- |
| `justino::window` | `window.create()`, `window.set_stylesheet()` | Interface gráfica desktop nativa acelerada por GPU. |
| `justino::http` | `http.listen()`, `http.get()`, `http.post()` | Servidor HTTP assíncrono e cliente web. |
| `justino::json` | `json.parse()`, `json.stringify()` | Serialização e parse de JSON nativo ultra-rápido. |
| `justino::fs` | `fs.read_file()`, `fs.write_file()`, `fs.load_env()` | I/O assíncrono com proteção contra Path Traversal e parser de `.env`. |
| `justino::crypto` | `crypto.hash_password()`, `crypto.sign_jwt()` | Hashing de senhas seguro (Argon2/Bcrypt/SHA256) e tokens JWT. |
| `justino::db` | `db.open()`, `db.query()` | Banco de dados SQLite embutido com proteção contra SQL Injection. |
| `justino::i18n` | `i18n.set_locale()`, `i18n.format_currency()` | Formatação internacional CLDR de moedas, decimais e datas. |

---

## 4. Language Server Protocol & Diagnósticos i18n (`justino_lsp`)

### Protocolo e Handlers
- **Processo Executável (`justino-lsp`)**: Binário ultra-rápido rodando via Stdio JSON-RPC 2.0 com < 20MB de RAM e < 5ms de latência.
- **Catálogo de Diagnósticos i18n**: Emissão de diagnósticos traduzidos no idioma do desenvolvedor (`pt-BR`, `en-US`, `es-ES`, `zh-CN`).
- **Autocompletar Inteligente & CSS Mapping**: Sugestão de palavras-chave, funções da Stdlib e classes CSS extraídas de arquivos `.css`.
- **Extrator de Contexto para IA (`context_builder`)**: Consolidação da AST do projeto, seletores CSS e documentação em JSON otimizado para IAs na IDE da Fase 5.

---

## 5. IDE Nativa Oficial PROMPT 05 DEFINITIVO (`justino_ide`)

### Arquitetura Clean UI/UX nível Apple / JetBrains / Xcode em `.jucode` + CSS3
- **Design System Clean**: Zero emojis, ícones vetoriais SVG minimalistas, grid 4px/8px e raio de bordas suaves (`6px`).
- **7 Menus Dropdown Globais Interativos**:
  - `File`: New File (`Ctrl+N`), Open (`Ctrl+O`), Save (`Ctrl+S`), Export Executable (`.exe`), Preferences.
  - `Edit`: Undo (`Ctrl+Z`), Redo (`Ctrl+Y`), Duplicate Line (`Shift+Alt+Down`), Find & Replace (`Ctrl+Shift+F`), Format Document (`Shift+Alt+F`).
  - `View`: Toggle Sidebar (`Ctrl+B`), Toggle Terminal (<code>Ctrl+`</code>), Toggle AI Panel (`Ctrl+L`), Change Theme, Live Preview UI.
  - `AI Tools`: AI Inline Refactor (`Ctrl+K`), Explain Selection, Generate Unit Tests, AI Code Review, Select Model (Claude 3.5 Sonnet, GPT-4o, Ollama Local).
  - `Run & Debug`: Run File (`F5`), Build Native Binary (`Ctrl+Shift+B`), Run Unit Tests.
  - `Tools & Extensions`: Marketplace Store, Install CSS Theme, LSP Server Status.
  - `Help`: Open Documentation, Keyboard Shortcuts, Check for Updates, About Justino Studio.
- **Catálogos i18n**: Tradução dos menus para `pt_BR`, `en_US`, `es_ES` e `zh_CN`.
- **Microsoft Monaco Editor Engine**: Tokenizador Monarch da sintaxe `.jucode`, minimapa, linha/coluna e visualizador ao vivo de interfaces UI + CSS com suporte RTL (`ar-SA`).

---

## 6. Registro de Decisões de Arquitetura (ADRs)

### ADR-001: Sintaxe Universal em Inglês
- **Decisão**: Palavras-chave da linguagem (`fn`, `let`, `mut`, `struct`, `async`, `await`, `return`, `if`, `else`, `match`, `spawn`, `import`, `export`, `try`, `catch`) utilizam o padrão internacional em **Inglês**.

### ADR-002: Internacionalização em Duas Camadas (Dev vs App)
- **Decisão**: A camada do compilador/LSP traduz diagnósticos no idioma do dev (ex: `pt-BR`), enquanto a camada de UI/Runtime trata CLDR/BCP 47 e Bidi LTR/RTL na aplicação.

### ADR-003: Proibição Absoluta de Código Inseguro (`unsafe`) e `panic!`
- **Decisão**: 0 `unsafe`, 0 `panic!`, 0 `unwrap()`, 0 `expect()` em todo o código de produção dos crates `justino_core`, `justino_ui`, `justino_stdlib`, `justino_lsp` e `justino_ide`.

---

## 7. Estado Atual do Projeto e Próximos Passos

| Subsistema | Estado | Testes | Cobertura |
| :--- | :--- | :--- | :--- |
| `justino_core` (Lexer, Parser, Compiler, VM) | ✅ Concluído | ✅ Passou (`cargo test`) | 100% Rust Seguro |
| `justino_ui` (CSS3, Flexbox, Bidi, Native Window) | ✅ Concluído | ✅ Passou (`cargo test`) | 100% Rust Seguro |
| `justino_stdlib` (Window, HTTP, JSON, FS, Crypto, DB, i18n) | ✅ Concluído | ✅ Passou (`cargo test`) | 100% Rust Seguro |
| `justino_lsp` (JSON-RPC 2.0, i18n Diagnostics, AI Context) | ✅ Concluído | ✅ Passou (`cargo test`) | 100% Rust Seguro |
| `justino_ide` (PROMPT 05 DEFINITIVO Clean IDE) | ✅ Concluído | ✅ Passou (`justino build`) | 100% .jucode + Rust |
| `docs/` (ARCHITECTURE, CONVENTIONS, ROADMAP, MEMORY) | ✅ Sincronizado | ✅ Verificado | Completo |
| `Phase 6` (Site Oficial, CI/CD GitHub Actions & Releases) | ⏳ Próxima Fase (Fase 6) | - | Pronto para receber Fase 6 |

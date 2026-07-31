# Justino Programming Language (`.jucode`) - Master Development Roadmap

---

## 📋 Checklist das 6 Fases do Ecossistema Justino

- [x] **Fase 0: Infraestrutura de Memória, Documentação e Diretrizes de i18n**
  - [x] Especificação da sintaxe universal em Inglês (`fn`, `let`, `mut`, `struct`, `async`, `await`, etc.).
  - [x] Criação e sincronização dos 4 arquivos de memória (`ARCHITECTURE.md`, `CONVENTIONS.md`, `ROADMAP.md`, `MEMORY.md`).
  - [x] Governança de Internacionalização (Camada Dev i18n e Camada App Bidi LTR/RTL).
  - [x] Diretrizes industriais de segurança em Rust (0 `unsafe`, 0 `panic!`, 0 `unwrap()`, 0 `expect()`).

- [x] **Fase 1: Núcleo do Compilador e VM em Registradores (`justino_core`)**
  - [x] Rastreamento de posição com `Span { file_id, start, end, line, column }`.
  - [x] Lexer UTF-8 (Scanner com suporte a Unicode, Emojis e Interpolação de Strings `"${expr}"`).
  - [x] Pratt Parser & AST completa resolvendo precedência de operadores.
  - [x] Compilador de Bytecode para Registradores e Emissão de Opcodes.
  - [x] VM baseada em Registradores e Gerenciador de Memória (`GcArena` + `Rc<RefCell<...>>`).

- [x] **Fase 2: Motor de UI Nativa, Parser de CSS3 e Suporte RTL (`justino_ui`)**
  - [x] Parser de CSS3 com suporte a especificidade `(id, class, tag)` e cascata de regras.
  - [x] Árvore de UI (`UiNode`) e Box Model (`content`, `padding`, `border`, `margin`).
  - [x] Motor de Layout Flexbox em Rust resolvendo dimensões principais e cruzadas.
  - [x] Motor Bi-Direcional Bidi (espelhamento automático LTR <-> RTL para `ar-SA`, `he-IL`).
  - [x] RenderContext GPU e Painter de primitivas retangulares e texto.
  - [x] Biblioteca de Widgets (`Window`, `Container`, `Text`, `Button`, `Input`).
  - [x] Aplicação integrada `app_demo.jucode` e `styles.css`.

- [x] **Fase 3: Biblioteca Padrão Nativa ("Batteries Included") (`justino_stdlib`)**
  - [x] Motor de Janela Nativa acelerada por GPU (`justino::window`).
  - [x] Servidor e Cliente HTTP/HTTPS assíncrono (`justino::http`).
  - [x] Serializador e Parser JSON nativo ultra-rápido (`justino::json`).
  - [x] Sistema de Arquivos assíncrono UTF-8 e leitor de `.env` (`justino::fs`).
  - [x] Criptografia de senhas (Argon2/Bcrypt/SHA256) e tokens JWT (`justino::crypto`).
  - [x] Banco de dados SQLite embutido com proteção contra SQL Injection (`justino::db`).
  - [x] Formatação internacional CLDR de moedas, números e datas (`justino::i18n`).
  - [x] Aplicação de demonstração `app_desktop.jucode`, `style.css` e `.env`.

- [x] **Fase 4: Language Server Protocol Multilingue (`justino_lsp`)**
  - [x] Servidor LSP nativo executável `justino-lsp` operando via JSON-RPC 2.0.
  - [x] Diagnósticos multilingue em tempo real no idioma do dev (`pt-BR`, `en-US`, `es-ES`, `zh-CN`).
  - [x] Autocompletar inteligente para Keywords, Stdlib e classes de seletores CSS.
  - [x] Documentação por passar o mouse (`hover`) e navegação "Go to Definition".
  - [x] Extrator de Contexto de Projeto para IA (`context_builder`) para a IDE.

- [x] **Fase 5: IDE Nativa Oficial (`justino_ide`)**
  - [x] Backend desktop nativo ultraleve em Rust (Tauri v2) com WebView GPU (< 80MB RAM).
  - [x] Wizard de Onboarding interativo de 3 etapas com login OAuth2 PKCE e chave JWT.
  - [x] Editor Monaco com sintaxe oficial `.jucode` e temas CSS dinâmicos (`dark_theme.css`, `cyberpunk_theme.css`).
  - [x] Agente de IA integrado com refatoração inline (Ctrl+K) e chat do projeto (Ctrl+L).
  - [x] Painel de Live Preview de softwares com UI e Loja de Extensões/Temas.

- [ ] **Fase 6: Site Oficial, Pipeline CI/CD Multiplataforma e Releases**
  - [ ] Pipeline CI/CD GitHub Actions compilando binários para Windows, Linux e macOS.
  - [ ] Documentação interativa e portal da linguagem Justino.

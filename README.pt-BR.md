# Motor de Streaming de Eventos Zero-Copy em Rust

[![Rust CI](https://github.com/FelipeMadson/rust-zero-copy-event-stream/actions/workflows/ci.yml/badge.svg)](https://github.com/FelipeMadson/rust-zero-copy-event-stream/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/badge/release-v1.0.0-green.svg)](https://github.com/FelipeMadson/rust-zero-copy-event-stream/releases)

Motor de streaming e enfileiramento de eventos em Rust baseado em buffer circular. Desenvolvido para aplicações de alto débito e latência na ordem de microssegundos.

## Recursos de Engenharia

- **Zero-Allocation**: Buffer pré-alocado na memória heap para evitar fragmentação contínua.
- **Tratamento de Erros Rastreável**: Erros tipados via `thiserror` sem risco de panics em tempo de execução.
- **Drenagem por Lotes**: Descarregamento em bloco (`batch flush`) altamente performático com transferência nativa de posse.

## Compilação e Testes

```bash
cargo build --release
cargo test
```

## Autor

Desenvolvido por **[Felipe Madison](https://github.com/FelipeMadson)**.

Live demo : https://aditya-r-m.github.io/twisty-tesseract

Building local clone :
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for [rust](https://www.rust-lang.org/tools/install)
- Run ```wasm-pack build --target web```
- Run ```python3 -m http.server &> /tmp/ttlog &```
- Open http://localhost:8000/

There are no project dependencies outside the standard rust toolchain, but the following tools are recommended for convenience:
- [Helix editor](https://helix-editor.com/) with [LSP integration](https://docs.helix-editor.com/languages.html) configured for [rust-analyzer](https://rust-analyzer.github.io/manual.html#rustup)
- [Tmux](https://github.com/tmux/tmux/wiki) split-pane session alias `tx` to trigger build on `Ctrl+B r`
```
alias tx='tmux new-session \; set -s escape-time 0 \; split-window -h -l 128 \; set -g status off \; bind-key r send-keys -t 0 "wasm-pack build --target web" Enter \; attach'
```
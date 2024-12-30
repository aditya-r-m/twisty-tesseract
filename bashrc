export CARGO_TERM_COLOR=never
alias tx='tmux new-session \; set -s escape-time 0 \; split-window -h -l 128 \; set -g status off \; bind-key r send-keys -t 0 "wasm-pack build --target web" Enter \; attach'


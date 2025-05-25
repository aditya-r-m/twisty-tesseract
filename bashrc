PS1='\[\033[01;38;5;4m\]\W \[\033[01;38;5;6m\]: \[\033[00m\]'
bind -s 'set completion-ignore-case on'
export PYTHONDONTWRITEBYTECODE=1

alias sd='cd $(find * -type d | fzf)'
alias gs='git status'
alias gd='git diff --word-diff'
alias ga='git add -A'
alias gc='git commit'
alias gp='git push'
alias gl='git log'
alias tx='TERM=xterm-256color tmux new-session \; set -s escape-time 0 \; split-window -h -l 128 \; set -g status off \; bind-key p send-keys -t 0 "python3 server.py &> /tmp/slog0 &" Enter \; bind-key q send-keys -t 0 "python3 -m http.server &> /tmp/slog1 &" Enter \; bind-key r send-keys -t 0 "wasm-pack build --target web" Enter \; attach'

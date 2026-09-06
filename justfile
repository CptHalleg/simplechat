default:
    @just --list

chat-server level="debug":
    RUST_LOG={{level}} cargo watch -x "run --package chat-server"

client level="debug":
    RUST_LOG={{level}} cargo watch -x "run --package client"

web:
    trunk serve

db:
    brew services start postgresql

db-reset:
    set -a
    source .env
    set +a
    psql \
        --username postgres \
        --dbname postgres \
        --set=db_password="$APP_USER_PASSWORD" \
        --file=init.sql

all:
    tmux kill-session -t simplechat 2>/dev/null || true
    tmux new-session -d -s simplechat
    tmux split-window -h -t simplechat
    tmux send-keys -t simplechat:0.0 'just client' C-m
    tmux send-keys -t simplechat:0.1 'just server' C-m
    tmux attach-session -t simplechat

stop:
    tmux kill-session -t simplechat

default:
    @just --choose

prepare:
    DATABASE_URL="postgres://lukas@localhost/simplechat_profile" cargo sqlx prepare --workspace

chat-server level="debug":
    RUST_LOG={{level}} cargo watch -x "run --package chat-server"

profile-server level="debug":
    RUST_LOG={{level}} cargo watch -x "run --package profile-server"

community-server level="debug":
    RUST_LOG={{level}} cargo watch -x "run --package community-server"

client level="debug":
    RUST_LOG={{level}} cargo watch -x "run --package client"

web:
    trunk serve

db:
    brew services start postgresql

db-reset-profile:
    . ./.env && \
    psql \
        --dbname postgres \
        --set=user_password="$PROFILE_USER_PASSWORD" \
        --file=profile-server/init.sql
    
    sqlx migrate run --source profile-server/migrations

all:
    tmux kill-session -t simplechat 2>/dev/null || true
    tmux new-session -d -s simplechat
    tmux split-window -h -t simplechat
    tmux send-keys -t simplechat:0.0 'just client' C-m
    tmux send-keys -t simplechat:0.1 'just server' C-m
    tmux attach-session -t simplechat

stop:
    tmux kill-session -t simplechat

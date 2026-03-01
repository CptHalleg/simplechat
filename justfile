default:
    @just --list

server level="info":
    RUST_LOG={{level}} cargo run --package server

client:
    cargo run --package client

all:
    @just server & just client

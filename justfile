set dotenv-load

server script *args:
    cargo {{script}} -p server {{args}}

entity script *args:
    cargo {{script}} -p entity {{args}}

migration script *args:
    cargo {{script}} -p migration {{args}}

build:
    just server build --release

migrate +args:
    just migration run -- {{args}}

run:
    npm i -g bunyan && just server run | bunyan

dev:
    npm i -g bunyan && cargo-watch -C crates/server -s 'cargo run | bunyan'

test *args:
    cargo test -p server {{args}}

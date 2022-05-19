set dotenv-load

migrate +args:
    cargo run -p migration -- {{args}}

run:
    npm i -g bunyan && cargo run -p server | bunyan

dev:
    npm i -g bunyan && cargo-watch -C crates/server -s 'cargo run | bunyan'

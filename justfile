set dotenv-load

test_log := env_var_or_default("TEST_LOG", "false")

build:
    cargo build -p server --release

start:
    docker compose up -d

start-rebuild:
    docker compose up -d --build server

stop:
    docker compose down

run *args:
    docker compose run -e TEST_LOG={{test_log}} -e S3__BASE_URL=http://s3:9000 server {{args}}

db +args:
    docker compose run postgres {{args}}

server script *args:
    just run cargo {{script}} -p server {{args}}

entity script *args:
    just run cargo {{script}} -p entity {{args}}

migration script *args:
    just run cargo {{script}} -p migration {{args}}

migrate +args:
    just migration run -- {{args}}

test *args:
    just run cargo nextest run -p server {{args}}

wipe:
    just stop || echo ""
    docker system prune --volumes -f

clippy:
    just run cargo clippy --tests --benches --all-targets --all-features -- -D warnings

fmt:
    just run cargo fmt --all

release:
    pnpm -C clients/ts run release && mv clients/ts/CHANGELOG.md CHANGELOG.md && git add . && git commit -m "chore: Update changelog" && git push
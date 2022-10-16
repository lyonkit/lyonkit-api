set dotenv-load

test_log := env_var_or_default("TEST_LOG", "false")

local-start:
  cargo run

local-migrate *args:
  cargo run -p migration -- {{args}}

local-fmt:
  cargo fmt --all

local-test *args:
    cargo nextest run -p server {{args}}

build:
    cargo build --release

start:
    docker compose up -d

start-rebuild:
    docker compose up -d --build server

stop:
    docker compose down

run *args:
    docker compose exec -e TEST_LOG={{test_log}} -e S3__BASE_URL=http://s3:9000 server {{args}}

db +args:
    docker compose exec postgres {{args}}

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

service-start-s3:
    docker run -e MINIO_ROOT_USER=lyonkit -e MINIO_ROOT_PASSWORD=lyonkit-s3-secret -p 9000:9000 -p 9001:9001 --name s3 --health-cmd "curl -f http://localhost:9000/minio/health/live" --health-interval 30s --health-retries 3 --health-timeout 20s -d bitnami/minio:latest

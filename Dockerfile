####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=lyonkit-api
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /lyonkit-api

COPY ./ .

RUN cargo build -p server --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /lyonkit-api

# Copy our build
COPY --from=builder /lyonkit-api/target/x86_64-unknown-linux-musl/release/lyonkit-api ./

# Use an unprivileged user.
USER lyonkit-api:lyonkit-api

CMD ["/lyonkit-api/lyonkit-api"]
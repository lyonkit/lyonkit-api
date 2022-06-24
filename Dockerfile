####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools musl-dev
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

RUN cargo build -p health-check --target x86_64-unknown-linux-musl --release
RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Build CA Certificates
####################################################################################################

FROM alpine:3.16.0 as ca-certificates
RUN apk add -U --no-cache ca-certificates

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import certificates
COPY --from=ca-certificates /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /lyonkit-api

# Copy our build
COPY --from=builder /lyonkit-api/target/x86_64-unknown-linux-musl/release/lyonkit-api ./
COPY --from=builder /lyonkit-api/target/x86_64-unknown-linux-musl/release/health-check ./

# Use an unprivileged user.
USER lyonkit-api:lyonkit-api

HEALTHCHECK --interval=30s --timeout=1s --start-period=2s --retries=3 CMD [ "/lyonkit-api/health-check" ]

ENV PORT=8080
EXPOSE $PORT

ENTRYPOINT ["/lyonkit-api/lyonkit-api"]
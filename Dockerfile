# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.43 as cargo-build

RUN apt-get update

WORKDIR /usr/src/upsb_four

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/upsb_four*

COPY src ./src
COPY build.rs .
COPY templates ./templates
COPY statics ./statics

RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM cargo-build

VOLUME ["/data"]
WORKDIR /data
COPY --from=cargo-build /usr/src/upsb_four/target/release/upsb_four /home/upsb_four/bin

CMD ["/home/upsb_four/bin"]
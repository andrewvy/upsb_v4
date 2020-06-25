# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.43 as cargo-build

RUN apt-get update

WORKDIR /usr/src/upsb_three

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/upsb_three*

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
COPY --from=cargo-build /usr/src/upsb_three/target/release/upsb_three /home/upsb_three/bin

CMD ["/home/upsb_three/bin"]
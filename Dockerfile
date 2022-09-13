FROM rust:1.63.0
WORKDIR /usr/src/siainfopersonalms
COPY . .
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN su
RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
# RUN apt-get update && apt-get install -y tree && tree .
# CMD ["sh", "-c", "python -m http.server 8000"]
CMD ["sh", "-c", "cargo run --release"]


# RUN cargo install --path . && cargo build --release && apt-get update && apt-get install -y tree && cd /usr/local/cargo/bin && tree .
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

# Build: docker build --progress=plain -t sia_info_personal_ms .
# Run: docker run -p 8000:8000 sia_info_personal_ms

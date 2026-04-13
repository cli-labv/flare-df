FROM rust:1.76-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        qpdf \
        ghostscript \
        ca-certificates \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

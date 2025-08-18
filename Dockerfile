# syntax=docker/dockerfile:1

FROM rust:1.87-bullseye AS builder

# Set work directory
WORKDIR /app

# Set environment variables for consistent builds
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.87.0 \
    CARGO_TERM_COLOR=always \
    RUST_BACKTRACE=1

# Install system dependencies required for the build
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    protobuf-compiler \
    git \
    curl \
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install specific Rust toolchain components
RUN rustup component add \
    rustfmt \
    clippy \
    llvm-tools-preview

# Install cargo tools needed for CI
RUN cargo install \
    cargo-llvm-cov \
    cargo-audit \
    cargo-outdated \
    cargo-deny

# Install pre-commit for code quality checks
RUN apt-get update && apt-get install -y python3 python3-pip \
    && pip3 install pre-commit \
    && rm -rf /var/lib/apt/lists/*

# Set up git safe directory (needed for actions/checkout in containers)
RUN git config --global --add safe.directory /app

# Copy project files
COPY . .

# Create cache directory for cargo
RUN mkdir -p /usr/local/cargo/registry

# Default command for CI
CMD ["cargo", "test"]

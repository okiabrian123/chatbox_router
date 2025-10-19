docker run -it --rm \
    --platform linux/amd64 \
    --mount type=bind,source=$(pwd),target=/app \
    -v cargo-cache:/root/.cargo \
    -v cargo-target:/cargo-target \
    my-rust-builder:latest \
    bash -c '
    cd /app && \
    temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]") && \
    export package_name=${temp#name=} && \
    export CARGO_TARGET_DIR=/cargo-target && \
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc && \
    export CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc && \
    apt-get update && apt-get install -y \
        gcc \
        g++ \
        gcc-multilib \
        g++-multilib \
        libssl-dev \
        pkg-config \
        curl \
        ca-certificates \
        default-libmysqlclient-dev \
        binutils-x86-64-linux-gnu && \
    chmod 777 /root/.cargo && \
    chmod 777 /cargo-target && \
    echo ${package_name} && \
    cargo build --release --target=x86_64-unknown-linux-gnu  && \
    cp /cargo-target/x86_64-unknown-linux-gnu/release/${package_name} /app/${package_name}-x86_64'
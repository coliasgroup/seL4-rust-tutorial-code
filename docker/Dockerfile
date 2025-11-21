#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

FROM debian:bookworm

RUN apt-get update -q && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        git \
        unzip \
        rsync \
        # for seL4
        gcc-aarch64-linux-gnu \
        python3-dev \
        python3-venv \
        cmake \
        ninja-build \
        device-tree-compiler \
        libxml2-utils \
        gcc-riscv64-unknown-elf \
        # for bindgen
        libclang-dev \
        # for test scripts
        python3-pexpect \
        python3-requests \
        # for book
        libssl-dev \
        pkg-config \
        linkchecker \
        # for hacking
        bash-completion \
        man \
        less \
        sudo \
    && \
    # install these without --no-install-recommends
    apt-get install -y \
        qemu-system-arm \
        qemu-system-misc \
    && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /tmp

ARG RUSTUP_CHANNEL

RUN curl -sSf https://sh.rustup.rs | \
        bash -s -- -y --no-modify-path \
            --default-toolchain $RUSTUP_CHANNEL \
            --component rustfmt \
            --component rust-src

ENV PATH=/root/.cargo/bin:$PATH

ENV SEL4_INSTALL_DIR=/opt/seL4

RUN set -eux; \
    git clone \
        https://github.com/seL4/seL4.git \
        --config advice.detachedHead=false; \
    cd seL4; \
    git checkout fdf7f2a0823330615dfb6b3ae6706403217e74ea;

RUN set -eux; \
    cd seL4; \
    python3 -m venv pyenv; \
    export PATH=$(realpath ./pyenv/bin):$PATH; \
    pip install tools/python-deps; \
    cmake \
        -DCMAKE_INSTALL_PREFIX=$SEL4_INSTALL_DIR \
        -DKernelPlatform=qemu-arm-virt \
        # TODO further develop kernel loader to make KernelArmHypervisorSupport=ON unecessary
        -DKernelArmHypervisorSupport=ON \
        -DKernelVerificationBuild=OFF \
        -DARM_CPU=cortex-a53 \
        -G Ninja \
        -S . \
        -B build; \
    ninja -C build all; \
    ninja -C build install; \
    rm -rf $(pwd);

RUN set -eux; \
    url="https://github.com/seL4/rust-sel4"; \
    rev="027e3eb33327a8dc69df4cd517694295e80574f3"; \
    common_args="--git $url --rev $rev --root $SEL4_INSTALL_DIR"; \
    CC_aarch64_unknown_none=aarch64-linux-gnu-gcc \
    SEL4_PREFIX=$SEL4_INSTALL_DIR \
        cargo install \
            -Z build-std=core,compiler_builtins \
            -Z build-std-features=compiler-builtins-mem \
            --target aarch64-unknown-none \
            $common_args \
            sel4-kernel-loader; \
    cargo install \
        $common_args \
        sel4-kernel-loader-add-payload;

ENV MICROKIT_SDK=/opt/microkit/sdk

RUN set -eux; \
    mkdir -p $MICROKIT_SDK; \
    cd $MICROKIT_SDK; \
    curl -L \
        "https://productionresultssa7.blob.core.windows.net/actions-results/797488c0-ac6a-4563-9d54-6f671ce673bd/workflow-job-run-b3b4c29a-375e-5034-871e-29135961456d/artifacts/edb0f20b7f086e40497fc5608d28b2b7b4fcc064e2edb84a9949250914ecbc03.zip?rscd=attachment%3B+filename%3D%22microkit-sdk-2.0.1-dev.160%2Ba7f037c-linux-x86-64.zip%22&se=2025-11-21T11%3A23%3A01Z&sig=IxPgp7f%2Bb6YgGeUrXuqLM3PRCVK42Zu14X8Qd3e4BoI%3D&ske=2025-11-21T21%3A10%3A00Z&skoid=ca7593d4-ee42-46cd-af88-8b886a2f84eb&sks=b&skt=2025-11-21T09%3A10%3A00Z&sktid=398a6654-997b-47e9-b12b-9515b896b4de&skv=2025-11-05&sp=r&spr=https&sr=b&st=2025-11-21T11%3A12%3A56Z&sv=2025-11-05" \
        -o sdk.zip; \
    unzip sdk.zip; \
    tar -xzf microkit-sdk-*.tar.gz --strip-components=1; \
    chmod -R a+X . # HACK \
    rm sdk.zip;

ARG UID
ARG GID

RUN set -eux; \
    if [ $UID -eq 0 ]; then \
        if [ $GID -ne 0 ]; then \
            echo "error: \$UID == 0 but \$GID != 0" >&2; \
            exit 1; \
        fi; \
    else \
        if getent passwd $UID; then \
            echo "error: \$UID $UID already exists" >&2; \
            exit 1; \
        fi; \
        if ! getent group $GID; then \
            groupadd --gid $GID x; \
        fi; \
        useradd --uid $UID --gid $GID --groups sudo --create-home x; \
    fi;

RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

USER $UID

RUN set -eux; \
    if [ $UID -ne 0 ]; then \
        curl -sSf https://sh.rustup.rs | \
            bash -s -- -y --no-modify-path \
                --default-toolchain $RUSTUP_CHANNEL \
                --component rustfmt \
                --component rust-src; \
    fi;

ENV PATH=/home/x/.cargo/bin:$PATH

RUN cargo install mdbook@0.4.36

ENV SHELL=/bin/bash

# TODO HACK
RUN set -eu; \
    f=~/.gitconfig; \
    echo '[safe]' > $f; \
    echo 'directory = /work/code' >> $f;

WORKDIR /work

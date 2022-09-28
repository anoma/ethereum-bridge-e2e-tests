# test runner image
# TODO: this image should be one built under the Anoma GitHub organization
# 08b4f5d6ab5e237b41f881aa2a9f9a35e99877a7 = eth-bridge-integration branch from 2022-09-27
FROM ghcr.io/james-chf/devchain-container:08b4f5d6ab5e237b41f881aa2a9f9a35e99877a7
ENV RUST_BACKTRACE=full
COPY build/debug/ wasm/
RUN ./init_chain.sh

COPY build/tests/ tests/

# test runner image
# TODO: this image should be one built under the Anoma GitHub organization
# a70eafb2a724f014eda0323a09ccb80af994cd8c = eth-bridge-integration branch from 2022-10-04
FROM ghcr.io/james-chf/devchain-container:a70eafb2a724f014eda0323a09ccb80af994cd8c
ENV RUST_BACKTRACE=full
COPY build/debug/ wasm/
RUN ./init_chain.sh

COPY build/tests/ tests/

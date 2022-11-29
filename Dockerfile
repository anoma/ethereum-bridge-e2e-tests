# test runner image
# TODO: this image should be one built under the Anoma GitHub organization
FROM ghcr.io/james-chf/devchain-container:ethbridge-v0.10.1
ENV RUST_BACKTRACE=full
RUN ./init_chain.sh

COPY build/tests/ tests/

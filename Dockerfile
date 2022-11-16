# test runner image
# TODO: this image should be one built under the Anoma GitHub organization
FROM ghcr.io/james-chf/devchain-container:ethbridge-2022-11-16-fixed-events-endpoint
ENV RUST_BACKTRACE=full
RUN ./init_chain.sh

COPY build/tests/ tests/

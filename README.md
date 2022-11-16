# ethereum-bridge-e2e-tests

A workspace for end-to-end tests for Namada's Ethereum bridge.

## Structure

Tests are built as self-contained binaries that should be possible to run against any compatible Namada node. Some tests may require the node is exposing an endpoint for direct submission of fake Ethereum bridge events, while other tests may work against a Namada chain and a real EVM chain which has the bridge deployed

#### Running locally using Docker Compose

To run all end-to-end tests against a preconfigured chain:

```shell
make docker
docker compose down  # to ensure any stale containers are removed
docker compose up
```

The `ledger` container runs indefinitely.

The `testrunner` container will run all tests in series. It will exit if a test fails (exits with status code 2) or errors (exits with status code 1), or once all tests have successfully passed (exited with status code 0).

There is an `adhoc` container that can be SSH'ed into if you want to run test binaries or interact with the ledger manually.

```shell
docker compose exec -it adhoc /bin/bash
```

The test network can be reset with `docker compose down`.

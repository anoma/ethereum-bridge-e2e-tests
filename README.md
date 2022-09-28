# ethereum-bridge-e2e-tests

A workspace for end-to-end tests for Namada's Ethereum bridge.

## Tests

### Unit and integration

```shell
cargo test
```

### End-to-end

End-to-end tests are binaries under `crates/e2e_tests/src/bin`. It should be possible to run them against any Namada chain which is running the Ethereum bridge in the appropriate mode. e.g. some tests may require validators to be exposing an endpoint for direct submission of fake Ethereum bridge events, while other tests may work against a Namada chain and a real EVM chain which has the bridge deployed.

#### Running locally using Docker Compose

> :warning: This method of running locally may work very slowly on Apple Silicon, due to emulation of the `linux/amd64` architecture

To run them against a preconfigured network:

```shell
make docker  # should be run any time test runners or wasms change
docker compose up
```

The `ledger` container runs indefinitely.

The `testrunner` container will run all tests in series. It will exit if a test fails (exits with status code 2) or errors (exits with status code 1), or once all tests have successfully passed (exited with status code 0).

There is an `adhoc` container that can be SSH'ed into if you want to run test binaries or interact with the ledger manually.

```shell
docker compose exec -it adhoc /bin/bash
```

The test network can be reset with `docker compose down`.

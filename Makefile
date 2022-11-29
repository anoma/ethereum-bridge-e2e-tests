cargo = $(env) cargo

e2e-test-binaries:
	RUSTUP_TOOLCHAIN="nightly-2022-11-03" $(cargo) -Z unstable-options \
		build \
			--target x86_64-unknown-linux-musl \
			--target-dir build/cache/x86_64-unknown-linux-musl \
			--package 'e2e_submit_fake_transfer' \
			--out-dir build/tests

docker: e2e-test-binaries
	docker compose build

.PHONY : e2e-test-binaries docker

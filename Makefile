VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

test:
	${CARGO} test ${VERBOSE} --all -- --nocapture

run:
	RUST_LOG=info,actix_web=debug,hyper=info,chat-app=debug${RUST_LOG} cargo run --bin chat-app --release

down-db:
	cd ./ops/docker && docker compose stop db && docker compose rm -f db

up-db:
	cd ./ops/docker && docker compose up db -d

check-fmt:
	cargo +nightly fmt ${VERBOSE} --all -- --check

fmt:
	cargo +nightly fmt ${VERBOSE} --all

clippy:
	${CARGO} clippy ${VERBOSE} --all --all-targets --all-features -- \
		-D warnings -D clippy::clone_on_ref_ptr -D clippy::enum_glob_use

ci: check-fmt clippy test
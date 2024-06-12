run:
	RUST_BACKTRACE=full RUST_LOG=info,actix_web=debug,hyper=info,broker=trace${RUST_LOG} cargo run --bin chat-app --release
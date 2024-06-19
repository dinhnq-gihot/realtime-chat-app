run:
	RUST_BACKTRACE=full RUST_LOG=info,actix_web=debug,hyper=info,chat-app=debug${RUST_LOG} cargo watch -x 'run --bin chat-app'
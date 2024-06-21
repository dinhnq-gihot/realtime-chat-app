run:
	RUST_LOG=info,actix_web=debug,hyper=info,chat-app=debug${RUST_LOG} cargo run --bin chat-app
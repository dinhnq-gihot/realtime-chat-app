run:
	RUST_LOG=info,actix_web=debug,hyper=info,chat-app=debug${RUST_LOG} cargo run --bin chat-app --release

down-db:
	cd ./ops/docker && docker compose stop db && docker compose rm -f db

up-db:
	cd ./ops/docker && docker compose up db -d
docker-build:
	docker build -t rust-grpc-exchange --no-cache -f ./docker/Dockerfile .
	docker tag rust-grpc-exchange rust-grpc-exchange:1.0.0

watch:
	cargo-watch -x 'run --bin server' -i 'logs'

lint:
	cargo fmt --all -- --check

test:
	cargo test --verbose

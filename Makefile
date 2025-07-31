setup:
	cargo install mdbook mdbook-alerts
	pnpm -r install
	pnpm i -g @redocly/cli

dev:
	cargo run --features develop


build-open-api:
	redocly build-docs docs/api/open-api.yml --output docs/mod-manual/src/openapi/open-api.html
	redocly build-docs docs/api/open-api.yml --output assets/mods/open-api/open-api.html

fix:
	cargo clippy --workspace --fix --allow-dirty
	cargo fmt --all

test:
	cargo test --workspace

# --- Variables ---
APP_NAME       := DesktopHomunculus
BIN_NAME       := desktop_homunculus
BUNDLE_DIR     := target/bundle
APP_BUNDLE     := $(BUNDLE_DIR)/$(APP_NAME).app
CONTENTS       := $(APP_BUNDLE)/Contents
DMG_FILE       := $(BUNDLE_DIR)/$(BIN_NAME).dmg
PLIST_TEMPLATE := build/macos/Info.plist
ICNS_FILE      := build/macos/AppIcon.icns
RUST_LICENSES_OUTPUT := credits/licenses/RUST_THIRD_PARTY.md
OPENAPI_OUTPUT ?= ../docs/website/static/api/open-api.yml

VERSION = $(shell cargo metadata --format-version 1 --no-deps 2>/dev/null \
	| jq -r '.packages[] | select(.name=="$(BIN_NAME)") | .version')

# --- CEF variables ---
BUNDLE_ID_BASE    := not.elm.homunculus.desktop
CEF_FRAMEWORK_DIR := $(HOME)/.local/share/Chromium Embedded Framework.framework
CEF_SENTINEL      := $(CEF_FRAMEWORK_DIR)/Chromium Embedded Framework

# --- Existing targets ---

.PHONY: setup setup-ci setup-cef debug gen-open-api fix-lint test gen-credits

setup:
	cargo install \
		"export-cef-dir@144.4.0+144.0.13" \
		"bevy_cef_debug_render_process@0.2.0" \
		"bevy_cef_render_process@0.2.0" \
		"bevy_cef_bundle_app@0.2.0" \
		"cargo-about"
	npm i -g @redocly/cli
	$(MAKE) setup-cef

setup-ci: ## CI release build setup (uses cargo-binstall, skips cargo-about and redocly)
	cargo binstall --no-confirm \
		"export-cef-dir@144.4.0+144.0.13" \
		"bevy_cef_debug_render_process@0.2.0" \
		"bevy_cef_render_process@0.2.0" \
		"bevy_cef_bundle_app@0.2.0"
	$(MAKE) setup-cef

setup-cef:
	@set -e; \
	if [ "$$(uname)" = "Darwin" ]; then \
		command -v export-cef-dir >/dev/null 2>&1 \
			|| { echo "ERROR: export-cef-dir not found. Run 'make setup' first."; exit 1; }; \
		if [ ! -f "$(CEF_SENTINEL)" ]; then \
			echo "==> Downloading CEF framework..."; \
			export-cef-dir --force "$(HOME)/.local/share"; \
		else \
			echo "==> CEF framework already installed. Skipping download."; \
		fi; \
		echo "==> Copying debug render process..."; \
		cp "$(HOME)/.cargo/bin/bevy_cef_debug_render_process" \
			"$(CEF_FRAMEWORK_DIR)/Libraries/bevy_cef_debug_render_process"; \
	else \
		echo "WARNING: CEF setup is not yet supported on $$(uname). Skipping CEF setup."; \
	fi

debug:
	cargo run --features develop

gen-open-api:
	cargo run -p homunculus_http_server --bin gen_openapi -- --output $(OPENAPI_OUTPUT)

fix-lint:
	cargo clippy --workspace --fix --allow-dirty
	cargo fmt --all

test:
	cargo test --workspace

gen-credits:
	mkdir -p credits/licenses
	cargo about generate --workspace --locked --config about.toml --output-file $(RUST_LICENSES_OUTPUT) about.hbs

# --- Release targets ---

.PHONY: release-macos release-macos-arm release-macos-x86 release-macos-universal bundle-macos-app package-macos-dmg bundle-macos-cef

release-macos: ## Build for native arch, bundle .app, create DMG
	cargo build --profile dist
	$(MAKE) bundle-macos-app BIN_SOURCE=target/dist/$(BIN_NAME)
	$(MAKE) package-macos-dmg

release-macos-arm: ## Build for Apple Silicon, bundle .app, create DMG
	SDKROOT=$$(xcrun -sdk macosx --show-sdk-path) \
		cargo build --profile dist --target=aarch64-apple-darwin --locked
	$(MAKE) bundle-macos-app BIN_SOURCE=target/aarch64-apple-darwin/dist/$(BIN_NAME)
	$(MAKE) package-macos-dmg

release-macos-x86: ## Build for Intel Mac, bundle .app, create DMG
	SDKROOT=$$(xcrun -sdk macosx --show-sdk-path) \
		cargo build --profile dist --target=x86_64-apple-darwin --locked
	$(MAKE) bundle-macos-app BIN_SOURCE=target/x86_64-apple-darwin/dist/$(BIN_NAME)
	$(MAKE) package-macos-dmg

release-macos-universal: ## Build universal binary, bundle .app, create DMG
	SDKROOT=$$(xcrun -sdk macosx --show-sdk-path) \
		cargo build --profile dist --target=aarch64-apple-darwin --locked
	SDKROOT=$$(xcrun -sdk macosx --show-sdk-path) \
		cargo build --profile dist --target=x86_64-apple-darwin --locked
	mkdir -p target/dist
	lipo -create -output target/dist/$(BIN_NAME) \
		target/aarch64-apple-darwin/dist/$(BIN_NAME) \
		target/x86_64-apple-darwin/dist/$(BIN_NAME)
	$(MAKE) bundle-macos-app BIN_SOURCE=target/dist/$(BIN_NAME)
	$(MAKE) package-macos-dmg

bundle-macos-app: ## Assemble .app bundle (internal: requires BIN_SOURCE)
	@if [ -z "$(BIN_SOURCE)" ]; then echo "ERROR: BIN_SOURCE not set"; exit 1; fi
	@echo "Bundling $(APP_NAME).app (version $(VERSION))..."
	rm -rf $(BUNDLE_DIR)
	mkdir -p $(CONTENTS)/MacOS
	mkdir -p $(CONTENTS)/Resources
	mkdir -p $(CONTENTS)/Frameworks
	sed 's/__VERSION__/$(VERSION)/g' $(PLIST_TEMPLATE) > $(CONTENTS)/Info.plist
	cp $(BIN_SOURCE) $(CONTENTS)/MacOS/$(BIN_NAME)
	cp -r assets $(CONTENTS)/Resources/assets
ifndef SKIP_GEN_CREDITS
	$(MAKE) gen-credits
endif
	cp -r credits $(CONTENTS)/Resources/credits
	cp $(ICNS_FILE) $(CONTENTS)/Resources/AppIcon.icns
	$(MAKE) bundle-macos-cef
	@echo "Done: $(APP_BUNDLE)"

package-macos-dmg: ## Create DMG from .app bundle (internal)
	@echo "Creating DMG..."
	rm -f $(DMG_FILE)
	mkdir -p $(BUNDLE_DIR)/dmg-staging
	cp -r $(APP_BUNDLE) $(BUNDLE_DIR)/dmg-staging/
	ln -s /Applications $(BUNDLE_DIR)/dmg-staging/Applications
	hdiutil create -fs HFS+ -volname "$(APP_NAME)" \
		-srcfolder $(BUNDLE_DIR)/dmg-staging $(DMG_FILE)
	rm -rf $(BUNDLE_DIR)/dmg-staging
	@echo "Done: $(DMG_FILE)"

bundle-macos-cef: ## Bundle CEF framework and helper apps into .app
	@echo "==> Bundling CEF framework..."
	bevy_cef_bundle_app \
		--app "$(APP_BUNDLE)" \
		--bundle-id-base "$(BUNDLE_ID_BASE)"
	@echo "==> CEF bundling complete."

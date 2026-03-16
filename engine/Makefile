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

# CUDA architecture for whisper-rs-sys (exported to env for CMake)
export CMAKE_CUDA_ARCHITECTURES

VERSION = $(shell cargo metadata --format-version 1 --no-deps 2>/dev/null \
	| jq -r '.packages[] | select(.name=="$(BIN_NAME)") | .version')

# --- Platform detection ---
UNAME_S    := $(shell uname -s)
IS_WINDOWS := $(findstring MINGW,$(UNAME_S))$(findstring MSYS,$(UNAME_S))

# Python command: python3 on macOS/Linux, python on Windows
ifeq ($(IS_WINDOWS),)
  PYTHON ?= python3
else
  PYTHON ?= python
endif

# --- CEF variables (used by release targets) ---
BUNDLE_ID_BASE    := not.elm.homunculus.desktop

# --- Existing targets ---

.PHONY: setup setup-ci setup-cef debug gen-open-api fix-lint test gen-credits

setup:
	$(PYTHON) scripts/setup.py

setup-ci: ## CI release build setup (uses cargo-binstall, skips cargo-about and redocly)
	$(PYTHON) scripts/setup_ci.py

setup-cef:
	$(PYTHON) scripts/setup_cef.py

debug:
	cargo run --features develop

# debug-cuda: CMAKE_CUDA_ARCHITECTURES = native
# debug-cuda:
# 	$(PYTHON) scripts/patch_whisper_cuda.py
# 	cargo run --features develop,stt-cuda

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

.PHONY: release-windows release-macos release-macos-arm release-macos-x86 release-macos-universal bundle-macos-app package-macos-dmg bundle-macos-cef

release-windows: ## Build Windows MSI installer
	$(PYTHON) scripts/release_windows.py

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

ifeq ($(OS),Windows_NT)
  IS_WINDOWS := 1
else
  UNAME_S    := $(shell uname -s)
  IS_WINDOWS := $(findstring MINGW,$(UNAME_S))$(findstring MSYS,$(UNAME_S))
endif

ifeq ($(IS_WINDOWS),)
  PYTHON ?= python3
else
  PYTHON ?= python
endif

.PHONY: setup debug test fix-lint gen-open-api \
        release-macos release-macos-arm release-macos-x86 release-macos-universal \
        bump-version check-version install-cli stage-runtime \
        build-openclaw-plugin install-openclaw-plugin

setup:
	pnpm install
	$(MAKE) -C engine setup

debug:
	pnpm build --filter '!docs'
	$(MAKE) -C engine debug


debug-cuda:
	pnpm build --filter '!docs'
	$(MAKE) -C engine debug-cuda
	
test:
	pnpm test
	$(MAKE) -C engine test

fix-lint:
	$(MAKE) -C engine fix-lint
	pnpm lint:fix

gen-open-api:
	$(MAKE) -C engine gen-open-api
	cd docs/website && npx docusaurus gen-api-docs homunculus
	pnpm build

release-windows:
	pnpm build
	$(MAKE) -C engine release-windows

release-macos:
	pnpm build
	$(MAKE) -C engine release-macos

release-macos-arm:
	pnpm build
	$(MAKE) -C engine release-macos-arm

release-macos-x86:
	pnpm build
	$(MAKE) -C engine release-macos-x86

release-macos-universal:
	pnpm build
	$(MAKE) -C engine release-macos-universal

install-cli: ## Install hmcs CLI via cargo install
	$(MAKE) -C engine install-cli

stage-runtime: ## Download and stage Node.js, pnpm, tsx for bundling
	$(MAKE) -C engine stage-runtime

bump-version:
	$(PYTHON) scripts/bump_version.py

check-version:
	$(PYTHON) scripts/bump_version.py --check

install-openclaw-plugin: 
	pnpm --filter @hmcs/openclaw-plugin build
	cd packages/openclaw-plugin && pnpm pack --out hmcs-openclaw-plugin.tgz
	openclaw plugins install --force --dangerously-force-unsafe-install packages/openclaw-plugin/hmcs-openclaw-plugin.tgz
	pnpm --filter @hmcs/openclaw-plugin exec rimraf hmcs-openclaw-plugin.tgz


UNAME_S    := $(shell uname -s)
IS_WINDOWS := $(findstring MINGW,$(UNAME_S))$(findstring MSYS,$(UNAME_S))

ifeq ($(IS_WINDOWS),)
  PYTHON ?= python3
else
  PYTHON ?= python
endif

.PHONY: setup debug test fix-lint gen-open-api \
        release-macos release-macos-arm release-macos-x86 release-macos-universal \
        bump-version check-version

setup:
	pnpm install
	$(MAKE) -C engine setup

debug:
	pnpm build
	$(MAKE) -C engine debug

test:
	pnpm test
	$(MAKE) -C engine test

fix-lint:
	$(MAKE) -C engine fix-lint

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

bump-version:
	$(PYTHON) scripts/bump_version.py

check-version:
	$(PYTHON) scripts/bump_version.py --check

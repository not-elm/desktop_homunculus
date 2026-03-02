.PHONY: setup debug test fix-lint gen-open-api \
        release-macos release-macos-arm release-macos-x86 release-macos-universal

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
	pnpm build

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

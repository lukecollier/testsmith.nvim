.PHONY: build build-release copy-libs clean help

help:
	@echo "testsmith-nvim build commands:"
	@echo ""
	@echo "  make build          Build debug binary"
	@echo "  make build-release  Build release binary and copy FFI libraries"
	@echo "  make copy-libs      Copy FFI libraries after building"
	@echo "  make clean          Remove build artifacts"
	@echo ""

build:
	cargo build

build-release:
	cargo build --release
	./scripts/copy-libs.sh

copy-libs:
	./scripts/copy-libs.sh

clean:
	cargo clean
	rm -rf lib/*/

MAGICK ?= magick

.PHONY: help build check test fmt fmt-fix clippy clean doc doc-open bench benchmark-images

help:
	@echo "jpeg2000-rs — common targets"
	@echo ""
	@echo "  make build             cargo build --release"
	@echo "  make check             cargo check --all-targets"
	@echo "  make test              cargo test"
	@echo "  make fmt               cargo fmt (check mode)"
	@echo "  make fmt-fix           cargo fmt (write)"
	@echo "  make clippy            cargo clippy with warnings denied"
	@echo "  make clean             cargo clean"
	@echo "  make doc               cargo doc --no-deps"
	@echo "  make doc-open          cargo doc --no-deps --open"
	@echo "  make bench             cargo bench (add benches first)"
	@echo "  make benchmark-images  regenerate PNGs under benchmark/images/"
	@echo ""
	@echo "Override ImageMagick binary: make benchmark-images MAGICK='magick convert'"

build:
	cargo build --release

check:
	cargo check --all-targets

test:
	cargo test

fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt --all

clippy:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean

doc:
	cargo doc --no-deps

doc-open:
	cargo doc --no-deps --open

bench:
	cargo bench

benchmark-images:
	mkdir -p benchmark/images
	$(MAGICK) -size 512x512 gradient:'#000000-#ffffff' -colorspace Gray -depth 8 \
		benchmark/images/gray_gradient_512.png
	$(MAGICK) -size 512x512 gradient:'#ff0000-#0000ff' -depth 8 \
		benchmark/images/rgb_gradient_512.png
	$(MAGICK) -size 256x256 pattern:checkerboard -depth 8 \
		benchmark/images/checker_256.png
	$(MAGICK) -size 384x256 xc:'gray(50%)' +noise Random -seed 42 -depth 8 \
		benchmark/images/noise_384x256.png
	-$(MAGICK) rose: -resize 512x512! benchmark/images/rose_512.png
	@if ! test -f benchmark/images/rose_512.png; then \
		echo "rose: unavailable; trying logo:"; \
		$(MAGICK) logo: -resize 512x512! benchmark/images/rose_512.png; \
	fi
	$(MAGICK) -size 64x64 gradient:'#000-#fff' -colorspace Gray -depth 8 \
		benchmark/images/gray_gradient_64.png

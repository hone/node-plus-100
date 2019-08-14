output_dir = output/rust-buildpack
dev_dir = target/debug

dev:
	cargo build
	rm -rf $(output_dir)
	mkdir -p $(output_dir)/bin
	cp $(dev_dir)/detect $(output_dir)/bin/
	cp $(dev_dir)/bin-build $(output_dir)/bin/build
	cp buildpack.toml $(output_dir)/

release:
	cargo build --release
	rm -rf $(output_dir)
	mkdir -p $(output_dir)/bin
	cp $(dev_dir)/detect $(output_dir)/bin/
	cp $(dev_dir)/bin-build $(output_dir)/bin/build
	strip $(output_dir)/bin/*
	cp buildpack.toml $(output_dir)/
	cd $(output_dir) && tar -pcvzf ../rust-buildpack-$$(git rev-parse --short HEAD).tar.gz .

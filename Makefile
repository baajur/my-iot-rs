# Please, follow the target naming conventions:
# https://www.gnu.org/prep/standards/html_node/Standard-Targets.html

EXECUTABLE_PATH := /usr/local/bin/my-iot

.PHONY: all
all:
	@RUSTFLAGS="-D warnings" cargo build --release

.PHONY: clean
clean:
	@cargo clean

.PHONY: check/clippy
check/clippy:
	@cargo clippy --workspace -- -D warnings

.PHONY: check/fmt
check/fmt:
	@cargo fmt -- --check

.PHONY: check/test
check/test:
	@cargo test --workspace

.PHONY: check
check: check/clippy check/fmt check/test

.PHONY: install
install:
	@cp target/release/my-iot $(EXECUTABLE_PATH)
	@setcap cap_net_raw+ep $(EXECUTABLE_PATH) || echo "Warning: install setcap to enable non-root ICMP"

.PHONY: uninstall
uninstall:
	@rm -f $(EXECUTABLE_PATH)

.PHONY: html docs
html docs:
	@cargo doc --document-private-items --no-deps
	@rsync -a --delete target/doc/ docs
	@echo '<html><head><meta http-equiv="refresh" content="0; url=my_iot"></head></html>' > docs/index.html

docker/build/%:
	@docker build -t "eigenein/my-iot-rs/$*" -f Dockerfile . --target "$*"
	@docker run --rm -v "$(PWD):/my-iot-rs" "eigenein/my-iot-rs/$*"

.PHONY: src/statics
src/statics:
	@curl 'https://cdnjs.cloudflare.com/ajax/libs/bulma/0.9.0/css/bulma.min.css' --output 'src/statics/bulma.min.css'
	@curl 'https://unpkg.com/bulma-prefers-dark@0.1.0-beta.0/css/bulma-prefers-dark.css' --output 'src/statics/bulma-prefers-dark.css'
	@curl 'https://cdn.jsdelivr.net/npm/chart.js@2.9.3' --output 'src/statics/chart.js'

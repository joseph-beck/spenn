CARGO ?= cargo
NPM ?= npm

.phony: api
api:
	$(CARGO) run --bin spenn-api

.phony: gui
gui:
	cd spenn-gui && $(NPM) run tauri dev

.phony: test
test:
	$(CARGO) test

.phony: fmt
fmt:
	$(CARGO) fmt

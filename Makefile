CARGO ?= cargo
NPM ?= npm
MAKE ?= make

.phony: api
api:
	$(MAKE) migrate
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

.phony: migrate
migrate:
	$(CARGO) run --bin migrate

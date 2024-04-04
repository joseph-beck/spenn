CARGO ?= cargo

.phony: api
api:
	$(CARGO) run --bin spenn-api

.phony: gui
gui:
	cd spenn-gui && npm run tauri dev

CARGO ?= cargo

.phony: api
api:
	$(CARGO) run --bin spenn-api

.phony: gui
gui:
	$(CARGO) run --bin spenn-gui

# pagers — Rust static site generator
# Common tasks: make help

CARGO ?= cargo
PAGERS ?= $(CARGO) run --release -p pagers-cli --
PORT ?= 8080

.PHONY: help build release run serve docs test lint fmt fmt-check clippy \
        ci examples clean install landing

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'

build: ## Debug build (workspace)
	$(CARGO) build

release: ## Release build of the pagers CLI
	$(CARGO) build --release -p pagers-cli

run: release ## Run pagers CLI: make run -- build docs --output dist
	./target/release/pagers $(ARGS)

serve: release ## Serve the docs content with pagers on :$(PORT)
	./target/release/pagers serve packages/landing/docs --port $(PORT)

landing: ## Build the landing site (Next.js static export) → packages/landing/out
	pnpm -C packages/landing install --frozen-lockfile
	pnpm -C packages/landing build

docs: landing ## Alias for landing

test: ## Run workspace tests
	$(CARGO) test --workspace

lint: clippy ## Alias for clippy

clippy: ## Clippy with warnings denied
	$(CARGO) clippy --workspace --all-targets -- -D warnings

fmt: ## Format all Rust code
	$(CARGO) fmt

fmt-check: ## Verify formatting (CI check)
	$(CARGO) fmt --check

ci: ## Reproduce the CI flow locally (./scripts/ci.sh)
	./scripts/ci.sh

ci-deploy: ## CI flow + stage .page and build _site (like a master push)
	./scripts/ci.sh --deploy

examples: ## Wikipedia crawl + demo site (see scripts/examples.sh --help)
	./scripts/examples.sh $(ARGS)

install: release ## Install pagers to ~/.local/bin
	cp target/release/pagers $${HOME}/.local/bin/pagers

clean: ## Remove build outputs and generated sites
	$(CARGO) clean
	rm -rf dist _site .page examples/demo

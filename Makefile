.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run-focused publish-dry-run-facade release-readiness facade-post-publish-validation verify

FOCUSED_CRATES := use-market-price use-return use-bar use-tick use-price-series use-volatility use-drawdown use-portfolio-weight use-factor use-signal-score use-risk
FACADE_CRATE := use-quant

help:
	@printf "%s\n" \
		"help                           Show available repository tasks" \
		"fmt                            Check formatting with rustfmt" \
		"check                          Run cargo check for the workspace" \
		"lint                           Run clippy with warnings denied" \
		"test                           Run workspace tests with all features" \
		"test-minimal                   Run workspace tests with no default features" \
		"build                          Build the workspace with all features" \
		"doc                            Build workspace docs without dependencies" \
		"examples                       Check all examples" \
		"audit                          Run cargo-audit" \
		"deny                           Run cargo-deny" \
		"sbom                           Generate a CycloneDX SBOM for $(FACADE_CRATE)" \
		"publish-dry-run-focused        List package contents and dry-run publish focused crates" \
		"publish-dry-run-facade         Dry-run publish $(FACADE_CRATE) after crates.io propagation" \
		"release-readiness              Run the pre-release focused-crate validation path" \
		"facade-post-publish-validation Dry-run the facade crate after focused crates are live" \
		"verify                         Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/$(FACADE_CRATE)/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run-focused:
	@if [ -z "$(strip $(FOCUSED_CRATES))" ]; then \
		printf "%s\n" "No focused crates configured"; \
	else \
		for crate in $(FOCUSED_CRATES); do \
			cargo package --list -p $$crate; \
			cargo publish --dry-run --allow-dirty -p $$crate; \
		done; \
	fi

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p $(FACADE_CRATE)

release-readiness: verify examples test-minimal publish-dry-run-focused

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build

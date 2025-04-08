test: check
	uv run maturin develop --release
	uv run pytest .

check:
	cargo fmt --check
	cargo clippy
	uv run ruff check
	uv run ruff format --check
	uv run mypy --strict .

test: check
	poetry run maturin develop --release
	poetry run pytest .

check:
	cargo fmt --check
	cargo clippy
	poetry check
	poetry run ruff check
	poetry run ruff format --check
	poetry run mypy --strict .

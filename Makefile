test: check
	poetry run maturin develop --release
	poetry run pytest -v .

check:
	cargo fmt --check
	cargo clippy
	poetry run isort . --check
	poetry run black --check .
	poetry run mypy --strict .

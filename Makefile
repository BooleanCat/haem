test: check
	poetry run maturin develop --release
	poetry run pytest .

check:
	cargo fmt --check
	cargo clippy
	poetry check
	poetry run isort . --check
	poetry run black --check .
	poetry run mypy --strict .

test: check
	poetry run maturin develop --release
	poetry run pytest -v .

check:
	poetry run black --check .
	poetry run mypy --strict .

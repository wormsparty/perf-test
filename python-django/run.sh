#!/bin/sh

export DB_HOST=localhost
export DB_PORT=5432
export DB_NAME=rust
export DB_USER=rust
export DB_PASSWORD=rust

if [ ! -d .venv ]; then
	python3 -m venv .venv
fi

. .venv/bin/activate
pip install -r requirements.txt
python manage.py runserver

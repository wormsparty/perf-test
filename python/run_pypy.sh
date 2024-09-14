#!/bin/sh

export DB_HOST=localhost
export DB_PORT=5432
export DB_NAME=rust
export DB_USER=rust
export DB_PASSWORD=rust

if [ ! -d .venv ]; then
	pypy3 -m venv .venv
fi

. .venv/bin/activate
pip install -r requirements_pypy.txt
pypy manage.py runserver

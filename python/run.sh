#!/bin/sh

export DB_HOST=localhost
export DB_PORT=5432
export DB_NAME=rust
export DB_USER=rust
export DB_PASSWORD=rust

. .venv/bin/activate
python manage.py runserver

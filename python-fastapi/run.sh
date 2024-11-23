#!/bin/sh

export $(cat ../.env | xargs)

if [ ! -d .venv ]; then
	python3 -m venv .venv
fi

. .venv/bin/activate
pip install -r requirements.txt
uvicorn app.main:app --reload
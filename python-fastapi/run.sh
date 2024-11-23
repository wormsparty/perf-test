#!/bin/sh

export $(cat ../.env | xargs)

if [ ! -d .venv ]; then
	python3 -m venv .venv
fi

. .venv/bin/activate
pip install -r requirements.txt
uvicorn src.main:app --workers 5 --log-level critical 
# For development use --reload instead of '--workers X'

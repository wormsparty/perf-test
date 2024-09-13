#!/bin/sh

if ! which multitime > /dev/null; then
	sudo apt install multitime
fi

multitime -q -n 128 -s 0 "./run_query.sh"

# TODO: Run everything

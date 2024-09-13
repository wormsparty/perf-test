#!/bin/sh

if ! which multitime > /dev/null; then
	sudo apt install multitime
fi

multitime -q -n 10 "./run.sh"

#!/bin/sh

cd "$(dirname "$0")" || exit

if [ -z "$GOPATH" ]; then
	export GOPATH="$HOME/go"
fi

if ! which go > /dev/null 2>&1; then
	for dir in "$HOME"/go/go*; do
		export PATH="$PATH:$dir/bin"
	done
fi

if [ ! -f "$GOPATH/bin/air" ]; then
	go install github.com/air-verse/air@latest
fi

"$GOPATH/bin/air" --build.cmd "./build.sh" --build.bin "./main"

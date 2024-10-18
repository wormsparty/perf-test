#!/bin/sh

cd "$(dirname "$0")" || exit

if [ -z "$GOPATH" ]; then
	export GOPATH="$PWD/gopath"
fi

if [ ! -f "$GOPATH/bin/air" ]; then
	go install github.com/air-verse/air@latest
fi

"$GOPATH/bin/air" --build.cmd "go build -o main ./src" --build.bin "./main"

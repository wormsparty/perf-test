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

go build -o main ./src

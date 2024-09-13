#!/bin/sh

if [ -z $GOPATH ]; then
	export GOPATH=$HOME/go
fi

if [ ! -f $GOPATH/bin/air ]; then
	go install github.com/air-verse/air@latest
fi

$GOPATH/bin/air --build.cmd "go build -o main ./src" --build.bin "./main"

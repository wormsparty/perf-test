#!/bin/sh

if ! which nest; then
	sudo npm i -g @nestjs/cli
fi

npm install

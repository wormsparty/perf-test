#!/bin/sh

cd "`dirname $0`"
export $(cat .env | xargs)
export PATH="$PATH:$HOME/.cargo/bin"

cargo install sea-orm-cli

sea-orm-cli generate entity \
    -u "${DATABASE_URL}" \
    -o src/entities

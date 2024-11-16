#!/bin/sh

cd "$(dirname "$0")" || exit
export "$(xargs < .env)"
export PATH="$PATH:$HOME/.cargo/bin"

cargo install sea-orm-cli

sea-orm-cli generate entity \
    --with-serde=serialize \
    -u "${DATABASE_URL}" \
    -o src/entities

#!/bin/sh

cd "`dirname $0`/.."
export $(cat .env | xargs)

psql $DATABASE_URL

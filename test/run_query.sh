#!/bin/sh

curl -X POST -H 'Content-Type: application/json' -d @data.json localhost:8000/api/list > /dev/null

if [ $? -ne 0 ]; then
	echo "ERREUR"
else
	echo "OK"
fi

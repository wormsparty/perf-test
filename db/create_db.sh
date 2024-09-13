#!/bin/sh

cd "`dirname $0`"

if ! which docker > /dev/null 2>&1; then
	echo "Doesn't isn't installed. Please do so with: "
	echo ""
	echo "    curl -sSL https://get.docker.com | sh"
	echo "    sudo usermod -aG docker $USER"
	echo ""
	echo "Then logout and log back in."
	echo ""
	exit 1
fi

if ! which psql > /dev/null 2>&1; then
	sudo apt install postgresql-client
fi

POSTGRES_PASSWORD=postgres
DB_NAME=rust
DB_USER=rust
DB_PASSWORD=rust

STATUS=$(docker container inspect -f '{{.State.Running}}' postgres)

if [ $? -ne 0 ]; then
	echo "Pulling image..."
	docker run --name postgres -d -p 5432:5432 -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD postgres:latest
	sleep 5
elif [ "$STATUS" = "false" ]; then
	echo "Starting container..."
	docker start postgres
	sleep 5
else
	echo "Container already running"
fi

DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME
ADMIN_DATABASE_URL=postgres://postgres:$POSTGRES_PASSWORD@localhost:5432

psql $ADMIN_DATABASE_URL << EOT
        CREATE USER $DB_USER with PASSWORD '$DB_PASSWORD' CREATEDB;
        CREATE DATABASE $DB_NAME WITH OWNER $DB_USER;
        GRANT postgres TO $DB_USER;
EOT

cat << EOT > ../.env
DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME
ADMIN_DATABASE_URL=postgres://postgres:$POSTGRES_PASSWORD@localhost:5432
EOT

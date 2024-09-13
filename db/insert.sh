#!/bin/sh

cd "`dirname $0`/.."
export $(cat .env | xargs)

psql $DATABASE_URL << EOT
CREATE TABLE entity (
	id serial primary key,
	colonne_1 varchar not null,
	colonne_2 varchar not null
);

do 
\$\$
begin
   for counter in 1..1000 loop
	   insert into entity (colonne_1, colonne_2)
	   select substr(md5(random()::text), 1, 25), substr(md5(random()::text), 1, 25);
   end loop;
end; 
\$\$;
EOT

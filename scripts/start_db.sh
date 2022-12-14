#!/bin/sh

PROJECT_ROOT="$(realpath $(pwd)/$(dirname $0)/..)"

CONTAINER_ID=$(
  docker run -it --rm --name gaia-auth-postgres-db \
    -e POSTGRES_DB=gaia \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=admin \
    -v "$PROJECT_ROOT/scripts/postgres-schema.sql:/docker-entrypoint-initdb.d/01-schema.sql" \
    -v "$PROJECT_ROOT/scripts/postgres-seeddata.sql:/docker-entrypoint-initdb.d/02-seed.sql" \
    -d postgres
)

CONTAINER_IP=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $CONTAINER_ID)
echo "DATABASE_URL=postgres://postgres:admin@$CONTAINER_IP/gaia" > "$PROJECT_ROOT/.env"
docker attach $CONTAINER_ID
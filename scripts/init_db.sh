#! /usr/bin/env bash

# This script is based on the book 'Zero to Production in Rust' by Luca
# Palmieri, see the original source here
# https://github.com/LukeMathWalker/zero-to-production/blob/main/scripts/init_db.sh

set -x
set -eo pipefail

# Load environment variables from .env file if present
if [[ -f ".env" ]]; then
  source .env
fi

# Check dependencies are present
if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "ERROR: Docker is not installed"
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "ERROR: sqlx is not installed"
    exit 1
fi

# Check if a custom parameter has been set, otherwise use default values
DB_PORT="${DB_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
APP_USER="${APP_USER:=app_user}"
APP_USER_PWD="${APP_USER_PWD:=password}"
APP_DB_NAME="${APP_DB_NAME:=crudkit_test_db}"
CONTAINER_NAME="${CONTAINER_NAME:=crudkit_db}"

# Launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        --env POSTGRES_USER=${SUPERUSER} \
        --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
        --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
        --health-interval=1s \
        --health-timeout=5s \
        --health-retries=5 \
        --publish "${DB_PORT}:5432" \
        --detach \
        --name "${CONTAINER_NAME}" \
        postgres -N 1000  # Increase maximum number of connections for testing purposes

    # Wait for DB to come up
    until [ \
        "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == "healthy" \
    ]; do
        >&2 echo "DB is not available yet, sleeping"
        sleep 1
    done

    # Create the application user
    CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
    docker exec -i "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

    # Grant create DB privileges to the app user
    GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
    docker exec -i "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
fi

>&2 echo "DB is up and running on port ${DB_PORT}"

# Create the application database and run migrations
DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "DB has been migrated, ready to go"

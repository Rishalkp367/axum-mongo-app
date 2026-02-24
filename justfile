# justfile for axum-mongo-app
# Convenience recipes for running the API and MongoDB during development.
#
# Usage:
#   just help
#   just compose-up
#   just run-host
#
# Notes:
# - The project loads its Mongo URI from MONGODB_URI (see .env when running in Docker Compose).
# - When running the binary on the host, use `just run-host` which defaults to 127.0.0.1.
# - When running everything with Docker Compose, the service name `mongo` is resolvable from the api container.

set shell := ["bash", "-cu"]

help:
    @echo "Available recipes:"
    @echo "  compose-up        - Build and start services with Docker Compose"
    @echo "  compose-down      - Stop and remove compose services"
    @echo "  run-mongo         - Run a standalone MongoDB container bound to localhost:27017"
    @echo "  run-host          - Run the API on the host and connect to a local MongoDB"
    @echo "  logs-mongo        - Tail logs from the mongo container"

compose-up:
    docker compose up --build

compose-down:
    docker compose down --volumes --remove-orphans

run-mongo:
    @echo "Starting a standalone MongoDB container bound to localhost:27017"
    docker run --name mongo_db -p 27017:27017 -d mongo:7

run-host mongo_uri="mongodb://127.0.0.1:27017" db_name="axum_db":
    @echo "Running API on host with MONGODB_URI={{ mongo_uri }} and MONGODB_DB={{ db_name }}"
    MONGODB_URI={{ mongo_uri }} MONGODB_DB={{ db_name }} cargo run

logs-mongo:
    @echo "Tailing logs from the mongo container (press Ctrl-C to stop)"
    docker logs -f mongo_db

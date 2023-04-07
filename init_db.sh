#!/bin/bash


# If docker database is not healthy, drop the database and start it again
if [ "$(docker inspect -f '{{.State.Health.Status}}' newsletter_db)" != "healthy" ];
then
  echo "Database is not healthy. Dropping database and starting it again..."
  # Drop the database
  docker-compose down
  # Start the database
  docker-compose up -d
fi

# Wait for the database and check if docker state is healthy
while [ "$(docker inspect -f '{{.State.Health.Status}}' newsletter_db)" != "healthy" ]; do
  echo "Waiting for database to be ready..."
  sleep 1
done

echo "Database is healthy and ready to use."
echo "Creating database and running migrations..."

# Create the database
DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
export DATABASE_URL
sqlx database create
sqlx migrate run

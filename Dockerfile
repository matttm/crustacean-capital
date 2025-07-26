#
#  purpose: docker container for a mysql db
#
#  docker build -t crust-db-img .
#
#  docker run -d -p 3306:3306 --name crust-db -e MYSQL_ROOT_PASSWORD=root --user mysql:mysql -v $(pwd)/assets:/var/lib/mysql crust-db-img
##
# Base image for MySQL
FROM mysql:8.0

# Set root password
ENV MYSQL_ROOT_PASSWORD=root

# Create a data volume for persistent storage
VOLUME ["/var/lib/mysql"]

# Expose port 3306
EXPOSE 3306

# Copy initialization script to be executed on startup
COPY init.sql /docker-entrypoint-initdb.d/


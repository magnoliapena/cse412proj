FROM postgres:latest

WORKDIR .

ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
ENV POSTGRES_DB docker

COPY ./classes-2214.csv .

COPY ./create_tables.sql /docker-entrypoint-initdb.d/

# COPY ./init.sh /docker-entrypoint-initdb.d/

VOLUME /tmp/postgres-data:/var/lib/postgresql/data

ENV PORT=5432

EXPOSE 5432

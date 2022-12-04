FROM postgres:latest

WORKDIR .

ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
ENV POSTGRES_DB postgres

COPY ./create_tables.sql /docker-entrypoint-initdb.d/

COPY ./classes.csv /var/lib/tabledata/

COPY ./populate_data.sh /docker-entrypoint-initdb.d/populate_data.sh

RUN chmod +x /docker-entrypoint-initdb.d/populate_data.sh

ENV PORT=5432

EXPOSE 5432

FROM postgres:latest

ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
ENV POSTGRES_DB todo

EXPOSE 5432

RUN groupmod -g 1000 postgres
RUN usermod -u 1000 postgres

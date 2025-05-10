FROM alpine

WORKDIR /app

COPY pocketbase .
COPY pb_migrations/ pb_migrations

RUN ls -al

RUN chmod +x pocketbase

RUN /app/pocketbase migrate
RUN /app/pocketbase superuser create admin@localhost.local localadmin

EXPOSE 8090
CMD ["/app/pocketbase", "serve", "--http", "0.0.0.0:8090"]
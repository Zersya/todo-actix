FROM debian:bullseye-slim
WORKDIR /app
ADD target/x86_64-unknown-linux-gnu/release/todo-actix .
CMD ["/app/todo-actix"]
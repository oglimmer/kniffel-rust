# Kniffel REST API - in Rust

This is a Rust rewirte of https://github.com/oglimmer/java-spring-boot-class-sources/tree/main/chapter06

It is fully compatible with the Vue frontend there

... and I should say that this is my first Rust project, so please give me some slack.

# build and run

you might need to install the mariadb/mysql client lib. it seemed to me that mysql v9 is not compatible.

```bash
# on macOS:
brew install mariadb-connector-c
export MYSQLCLIENT_VERSION=3.4.1
export MYSQLCLIENT_LIB_DIR=/opt/homebrew/Cellar/mariadb-connector-c/3.4.1/lib/ # make sure this is your path too
```

start a DB

```bash
docker run -d --rm -e MARIADB_ROOT_PASSWORD=root -e MARIADB_USER=kniffel \
    -e MARIADB_PASSWORD=kniffel -e MARIADB_DATABASE=kniffel -p 3306:3306 mariadb
```

start the REST API

```bash
cargo run
```

See OpenAPI at http://localhost:8080/swagger-ui/index.html

# docker compose

```bash
docker compose up --build -d
```

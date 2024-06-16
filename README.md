# Rocket Template

This repository is a template for building APIs in Rust with Rocket.

## Stack

- Rocket
- Diesel
- Sqlite

## Features

Implemented:

- Two models : User and Message
- Routes to create, get, update, delete them
- Routes to catch 404 and serve statics
- Integration tests
- Sqlite connection pool
- Authentication by JWT

## Usage

### Run the API:

With docker:

```bash
docker-compose up -d --build
```

Without:

```bash
ROCKET_DATABASES='{rocket_template_db={url="db.sqlite"}}' cargo run
```

### Run tests:

```bash
cargo test
```

## Architecture : `tree -I target`

```
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── README.md
├── db.sqlite
├── diesel.toml
├── docker-compose.yml
├── src
│   ├── bin
│   │   └── main.rs
│   ├── db
│   │   ├── common.rs
│   │   ├── message.rs
│   │   └── user.rs
│   ├── lib.rs
│   ├── logic
│   │   ├── home.rs
│   │   ├── message.rs
│   │   └── user.rs
│   ├── migrations
│   │   ├── 2024-06-15-081703_create_tables
│   │   │   ├── down.sql
│   │   │   └── up.sql
│   │   └── 2024-06-16-172322_add_user_name_and_password_in_users
│   │       ├── down.sql
│   │       └── up.sql
│   ├── models
│   │   ├── message.rs
│   │   └── user.rs
│   ├── routes
│   │   ├── home.rs
│   │   ├── message.rs
│   │   └── user.rs
│   ├── schema.rs
│   ├── static
│   │   └── favicon.ico
│   ├── structs
│   │   ├── jwt.rs
│   │   ├── request.rs
│   │   └── response.rs
│   └── utils
│       ├── common.rs
│       └── jwt.rs
└── tests
    ├── mod.rs
    ├── routes
    │   ├── home.rs
    │   ├── message.rs
    │   └── user.rs
    └── utils
        └── mod.rs
```

## Support

Please [open an issue](https://github.com/thomassimmer/rocket-template/issues/new/) for
support.

## Contributing

Please contribute using [Github Flow](https://guides.github.com/introduction/flow/). Create a branch, add commits, and [open a pull request](https://github.com/thomassimmer/rocket-template/compare).

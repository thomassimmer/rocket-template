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
- Websocket to listen for the creation of new messages
- Worker to handle the generation of answers to messages

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
JWT_SECRET='foo' cargo test
```

## Architecture : `tree -I target`

```
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── README.md
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
│   │   ├── message_channel.rs
│   │   └── user.rs
│   ├── schema.rs
│   ├── static
│   │   └── favicon.ico
│   ├── structs
│   │   ├── app_state.rs
│   │   ├── jwt.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   ├── task.rs
│   │   └── worker_queue.rs
│   └── utils
│       ├── common.rs
│       ├── jwt.rs
│       └── worker.rs
└── tests
    ├── mod.rs
    ├── routes
    │   ├── home.rs
    │   ├── message.rs
    │   └── user.rs
    └── utils
        └── test_context.rs
```

## Articles on Medium

I wrote these two articles on Medium about this projet:

- [Rust: Writing Tests in Rocket using an In-Memory DB 🚀](https://medium.com/@thomas.simmer/rust-writing-tests-in-rocket-49dd1733350e)
- [Rust: Build a Simple Celery-like Worker](https://medium.com/@thomas.simmer/rust-build-a-simple-celery-like-worker-7ae90f170515)

## Support

Please [open an issue](https://github.com/thomassimmer/rocket-template/issues/new/) for
support.

## Contributing

Please contribute using [Github Flow](https://guides.github.com/introduction/flow/). Create a branch, add commits, and [open a pull request](https://github.com/thomassimmer/rocket-template/compare).

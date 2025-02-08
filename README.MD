# Simple CRUD using Actix Web and PostgreSQL

This project is a simple CRUD (Create, Read, Update, Delete) API built using the Actix Web framework, SQLx and PostgreSQL as the database.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Cargo (Rust package manager)

## Getting Started

### Clone the repository

```sh
git clone https://github.com/singgihpr/actix_simple_crud_postgres.git
cd actix_simple_crud_postgres
```

### Set up the database

1. Install PostgreSQL and start the service.
2. Create a new database:

```sh
psql -U postgres
CREATE DATABASE rust_crud_postgres;
```

3. Set up the database schema:

```sh
psql -U postgres -d rust_crud_postgres -f schema.sql
```

### Configure environment variables

Create a `.env` file in the root directory and add the following:

```env
DATABASE_URL=postgres://username:password@localhost/rust_crud_postgres
```

Replace `username` and `password` with your PostgreSQL credentials.

### Build and run the project

```sh
cargo build
cargo run
```

The server will start at `http://localhost:8080`.

## API Endpoints

- `GET /users` - Retrieve all users
- `GET /users/{id}` - Retrieve an users by ID
- `POST /users` - Create a new users
- `PATCH /users/{id}` - Update an users by ID
- `DELETE /users/{id}` - Delete an users by ID

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Actix Web](https://actix.rs/)
- [PostgreSQL](https://www.postgresql.org/)

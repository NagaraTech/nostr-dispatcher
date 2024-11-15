# Nostr Dispatcher

Nostr Dispatcher is a Rust-based server application that handles message post by user and disaptch it to relays.

## Features

- Store the message and dispatch it to nostr relay

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL

### Installation

1. Clone the repository:

```
git clone https://github.com/your-username/nostr-dispatcher.git
cd nostr-dispatcher
```

2. Set up the database:
   - Create a PostgreSQL database
   - Set the `DATABASE_URL` environment variable in a `.env` file:

```
DATABASE_URL=postgres://username:password@localhost/database_name
```

3. Run database migrations:

```
diesel migration run
```

4. Build and run the project:

```
cargo run
```

## Project Structure

The main components of the project are:

- `src/main.rs`: Entry point of the application
- `src/config.rs`: Configuration settings
- `src/schema.rs`: Database schema definitions

## API Endpoints

- `/api/message/submit`: submit the message
- `/api/message/list`: get the message which user send to the dispatcher
- `/api/message/record`: get the record which dispatcher send to relay

- `/api/relay/register`: register a relay
- `/api/relay/remove`: remove a relay
- `/api/relay/list`: rget the realy list

## Configuration

Edit the config `dispatcher.toml`

## Database

The project uses Diesel ORM with PostgreSQL. The database schema is defined in `src/schema.rs`, and migrations are located in the `migrations/` directory.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).

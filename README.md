# Rust Notes CLI

**Rust Notes CLI** is a command-line application written in Rust, designed for managing and maintaining notes. It utilizes the PostgreSQL database for data storage.

## Versions
- PostgreSQL: 14.1
- Rust: 1.73.0

## Setup
Before using the **Rust Notes CLI** application, make sure to set up the necessary configuration in an `.env` file.

### Configuration
Create an `.env` file in the project directory and define the following environment variables:

- **DEBUG**: Set this variable to 1 if you want to enable debug lines for improved visibility during application execution.

- **DATABASE_URL**: Enter the PostgreSQL connection URL, including the appropriate credentials and connection details.

- **DATABASE_NAME**: Specify the name of the PostgreSQL database where your notes will be stored.

- **TABLE_NAME**: Define the name of the table within the specified database to store your notes.

Example .env file content:
```shell
DEBUG=1
DATABASE_URL=postgres://username:password@localhost:5432/your_database
DATABASE_NAME=your_database
TABLE_NAME=your_table
```

## Usage
To initialize the application and set up the database, run the application with the --init flag:

`./rust-notes-cli --init`

This command will configure the necessary database and table structure based on the provided environment variables, allowing you to start using the Rust Notes CLI with ease.

To add a note
`./rust-notes-cli --add --title {your title here} --details {your details here}`

This command will add a new note entry.

To list notes
`./rust-notes-cli --list --limit {optionally limit the search result by passing an integer}`

Get started and manage your notes efficiently with Rust Notes CLI!



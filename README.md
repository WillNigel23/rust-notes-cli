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

### Initialization

To initialize the application and set up the database, run the application with the --init flag:

`./rust-notes-cli --init`

This command will configure the necessary database and table structure based on the provided environment variables, allowing you to start using the Rust Notes CLI with ease.

### Adding a Note

To add a new note, you can use the following command:

`./rust-notes-cli --add --title "Your Title" --details "Your Details"`

Replace "Your Title" and "Your Details" with the actual title and details of your note. This command will create a new note with the specified title and details.

### Listing Notes

You can list your notes using the following command:

`./rust-notes-cli --list`

This command will display up to 10 notes by default, sorted from the latest to the oldest. To limit the number of notes displayed, you can use the --limit option, as shown in the next example.

`./rust-notes-cli --list --limit 5`

This will limit the list to the 5 most recent notes.

### Deleting a Note

To delete a specific note, you can use the following command:

`./rust-notes-cli --delete {id}`

Replace {id} with the actual ID of the note you want to delete. This command will remove the specified note from your notes database.

Get started and manage your notes efficiently with Rust Notes CLI!

## TODO

- Filtering lists

- Editing Notes

- Refactoring DB Connections



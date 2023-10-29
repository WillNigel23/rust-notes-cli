extern crate postgres;
extern crate dotenv;
extern crate structopt;
mod utils;

use postgres::{Client, NoTls};

use dotenv::dotenv;
use std::env;

use structopt::StructOpt;

use utils::pd;

#[derive(StructOpt)]
struct Opt  {
    #[structopt(short, long)]
    init: bool,
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");

    pd("Connecting to Postgres");
    let mut client = match connect_db(&database_url) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error connecting to the database: {}",e);
            return;
        }
    };
    pd("Connected to Postgres");

    let opt = Opt::from_args();

    if opt.init {
        init(&mut client);
    }
}

fn init(client: &mut Client) {
    pd("Initializing");


    pd("Checking Database Existence");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set in .env");
    let database_exists: bool = client
        .query_one(
            "SELECT EXISTS (SELECT FROM pg_database WHERE datname = $1)",
            &[&database_name],
        )
        .unwrap()
        .get(0);

    if database_exists {
        pd("Database Already Exist");
    }
    else {
        match client.simple_query(&format!("CREATE DATABASE {}", database_name)) {
            Ok(_) => pd("Database Created Successfully"),
            Err(e) => eprintln!("Error creating database: {}", e),
        }
    }

    pd("Checking if table exist");
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME not set in .env");
    let table_exists: bool = client
        .query_one(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = $1)",
            &[&table_name],
        )
        .unwrap()
        .get(0);

    if table_exists {
        pd("Table exists");
        pd("Checking integrity of table");
        let column_data = client.query(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = $1",
            &[&table_name],
            );

        let mut structure_valid = false;

        for row in &column_data.unwrap() {
            let column_name: String = row.get("column_name");
            let data_type: String = row.get("data_type");

            if (column_name == "title" || column_name == "details" || column_name == "time")
                && data_type == "character varying"
            {
                structure_valid = true;
            } else {
                structure_valid = false;
                break;
            }
        }

        if structure_valid {
            pd("Table structure is correct.");
        } else {
            pd("Table structure incorrect. Fixing structure");
            client.batch_execute("DROP TABLE notes").ok();
            create_table(client);
        }
    }
    else {
        create_table(client);
    }

    pd("Setup complete");
}


fn connect_db(database_url: &str) -> Result<Client, postgres::error::Error> {
    let client = Client::connect(database_url, NoTls)?;

    Ok(client)
}

fn create_table(client: &mut Client) {
    pd("Creating Table");
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME not set in .env");
    let create_table_sql = format!(
        r#"
        CREATE TABLE IF NOT EXISTS {} (
            title VARCHAR,
            details VARCHAR,
            time VARCHAR
        )
        "#,
        table_name
    );

    if let Err(err) = client.batch_execute(&create_table_sql) {
        eprintln!("Error creating '{}' table: {}", table_name, err);
        return;
    }

    pd("Table Created");
}


use serde::{Deserialize};
use std::{env, process::{Command, exit}, fs::File, io::Write};

#[derive(Debug, Deserialize)]
struct Field {
    label: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Item {
    fields: Vec<Field>,
}

struct Credential {
    label: String,
    value: String,
}

fn main() {
    // Retrieve the item name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the item name as a command-line argument");
        return;
    }
    let item_name = &args[1];

    // Run the 1Password command-line tool (`op`) to fetch the credentials
    let output = Command::new("op")
        .arg("item")
        .arg("get")
        .arg(item_name)
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute 1Password command");

    // Check the exit status of the `op` command
    if !output.status.success() {
        eprintln!("Unable to find item in 1Password");
        exit(1)
    }

    // Parse the JSON output
    let json_output = String::from_utf8_lossy(&output.stdout);
    
    let op_response: Result<Item, _> = serde_json::from_str(&json_output);

    match op_response {
        Ok(item) => {
            // Filter and map fields to credentials
            let credentials: Vec<Credential> = item.fields
                .into_iter()
                .filter_map(|field| field.value.map(|value| Credential { label: field.label, value }))
                .collect();

            // Create a .env file and write the credentials
            write_env_file(&credentials);

            eprint!("Credentials downloaded and saved to .env file")
        }
        Err(err) => {
            eprintln!("Failed to parse JSON output: {}", err);
        }
    }
}

fn write_env_file(credentials: &[Credential]) {
    // Create a .env file and write the credentials
    let mut file = File::create(".env").expect("Failed to create .env file");

    for credential in credentials {
        let line = format!("{}={}\n", credential.label, credential.value);
        file.write_all(line.as_bytes()).expect("Failed to write to .env file");
    }
}

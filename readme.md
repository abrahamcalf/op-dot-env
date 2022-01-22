# op-dot-env

> A Rust CLI tool to interact with 1Password and download credentials for environment variables.

## Features

- Connects to 1Password using the `op` command-line tool.
- Retrieves the specified item from 1Password.
- Parses the JSON response and extracts the required credentials.
- Creates a `.env` file with the extracted credentials.

## Prerequisites

- Rust programming language and Cargo package manager installed.
- 1Password command-line tool (`op`) installed and configured.

## Usage

2. Navigate to the project directory:

```shell
cd op-dot-env
```

   *Note: Make sure you have the necessary environment variables set up to connect to 1Password.*

3. Build the project:

```shell
cargo build --release
```

   This will compile the project and create the binary file.

4. Run the CLI tool with the desired item name:

```shell
./target/release/op-dot-env <item-name>
```

   Replace `<item-name>` with the name of the 1Password item containing the credentials you want to download.

5. The tool will connect to 1Password, fetch the credentials, and create a `.env` file in the current directory.

6. Open the `.env` file to access the downloaded credentials for your environment variables.

## License

This project is licensed under the [MIT License](LICENSE).

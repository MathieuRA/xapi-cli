# XAPI CLI

This is a command-line interface (CLI) for interacting with the Xen API (XAPI). It allows you to execute XAPI methods, optionally providing parameters, in a simple interactive manner.

## TODO for the V1

- [ ] Handle arrow up/down/left/right
- [x] Retry if invalid password
- [x] Help method that display the helper
- [ ] Handle args passed in a string. ["opaqueRef:.."] and [opaqueRef:..] should not throw any error
- [x] Better display of Success/Error.
- [ ] Use async call to avoid timeout issues
- [ ] Release the V1

## Getting Started

To use this CLI, you'll need to have Rust installed on your system. If you haven't already, you can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

1. Clone this repository to your local machine.
2. Navigate to the repository's directory.
3. Build the project by running:

   ```
   cargo build --release
   ```

4. After the build is successful, the compiled binary will be available in the `target/release` directory. You can run the XAPI CLI using this binary:

   ```
   ./target/release/xapi-cli --url <URL> -p <password>
   ```

   Replace `<URL>` and `<password>`. For example:

   ```
   ./target/release/xapi-cli --url http://192.168.1.12 -p somepassword
   ```

The CLI will prompt you to enter [XAPI methods](https://xapi-project.github.io/xen-api/). You can provide parameters if the method requires them. Type 'exit' to quit the CLI.

## Dependencies

This project uses the following Rust crates:

- `xmlrpc`: For XML-RPC communication with the Xen API.
- `clap`: For command-line argument parsing.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

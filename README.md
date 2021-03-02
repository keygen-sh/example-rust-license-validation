# Example Rust License Validation
This is an example Rust program for validating a license key. It contains a
small command line program which prompts for a license key value and then
validates the key using Keygen's software licensing API.

## Running the example

First up, configure a couple environment variables:

```bash
# Your Keygen account ID
export KEYGEN_ACCOUNT_ID="YOUR_KEYGEN_ACCOUNT_ID"
```

You can either run each line above within your terminal session before
starting the app, or you can add the above contents to your `~/.bashrc`
file and then run `source ~/.bashrc` after saving the file.

Next, install dependencies with [`cargo`](https://doc.rust-lang.org/cargo/):

```
cargo build
```

Then run the program:

```
cargo run
```

## Questions?

Reach out at [support@keygen.sh](mailto:support@keygen.sh) if you have any
questions or concerns!

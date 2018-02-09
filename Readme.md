## Description
Rusty razor is (intended to be) a Rust implementation of the Razor view engine.

The current plan is to simply parse the `.rs.html` files, and generate Rust code which can then be compiled as a separate step.
Eventually, I would like to write a dynamic loader which can be compiled into a web app for testing purposes, which would watch
for changes to the `.rs.html` files and recompile them on the fly without needing to stop and restart the application.

## Building
To build the application and run the tests, use the following:
```
cargo run -- -d tests # Generates the .rs files for the tests
cargo test
```

## Disclaimer
Note that this is primarily a learning exercise for me.  I do not claim that this is optimal code, that it represents good Rust
coding guidelines, or that it even works at all.

License: This work is licensed under the MIT license.  See the License file for further details.

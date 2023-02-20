# Rust-SMS-Service
This Rust service sends an SMS using Twilio. 

This code uses the reqwest crate to make an HTTP request to the Twilio API. It creates a new Client object and uses it to send a POST request to the Messages.json endpoint with the necessary parameters for sending an SMS message. If the response is successful, the code prints a success message. Otherwise, it prints an error message with the response text.


# Configuration Steps

Create a Virtual Environment
The purpose of virtual environments is to create a self-contained environment for each of your projects, allowing you to manage dependencies, libraries, and versions separately for each project.

`python3 -m venv rustenv`

`source rustenv/bin/activate`

`cd rustenv`
Install Rust Go to https://rustup.rs/ and run the command curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Run source "$HOME/.cargo/env" to configure your current shell.

create new project The cargo tool is the default package manager for Rust and provides an easy way to manage dependencies and build projects.

Run cargo new (project name) (my Eg: cargo new hello)

This will create a binary (application) microservice package

Create main.rs and lib.rs files in the src project

touch main.rs and touch lib.rs

Run Cargo build
This is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The cargo build command can be run from the root directory of the project.

Set up Cargo.toml to determine the dependencies and build configuration of the project.


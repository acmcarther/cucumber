# Cucumber-rs

[![Build Status](https://travis-ci.org/acmcarther/cucumber-rs.svg?branch=master)](https://travis-ci.org/acmcarther/cucumber-rs)
[Documentation](https://acmcarther.github.io/cucumber/cucumber/index.html)

## Setup for your local Rust project
In your rust project:
- Add the cucumber-rs dependency to your Cargo.toml
- Add a gemfile containing `gem 'cucumber'`, preferably with the same ruby version as the example
- Add a feature directory, containing the following basic files
  - features/cuke.rs (based off the [example](examples/calculator/features/cuke.rs))
  - features/step_definitions/cucumber_rust.wire (based off the [example](examples/calculator/features/step_definitions/cucumber_rust.wire))
- Add a cargo test entry to your Cargo.toml to hook cucumber tests into `cargo test`(based off [example](examples/calculator/Cargo.toml))
- Run cargo test, and verify that cucumber executes with no steps
- Add your `*.feature` files and step defintions as normal

## Running the Cucumber tests for this project
`cargo test`

## Running the Cucumber tests for the example
Inside examples/calculator
`cargo test`

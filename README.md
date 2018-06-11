# Cucumber (for Rust)

## Looking for maintainers!

[![Build Status](https://travis-ci.org/acmcarther/cucumber.svg?branch=master)](https://travis-ci.org/acmcarther/cucumber)

[Documentation](https://acmcarther.github.io/cucumber/cucumber/index.html)

![example run](https://cloud.githubusercontent.com/assets/1660129/14127154/a6703cfa-f5ca-11e5-998b-a7919eac8a75.gif)

## Before Going Further
Please checkout the [calculator example](examples/calculator)! That example is a full cargo project example showcasing Cucumber tests on real-ish behavior, and outputting a binary. It'll hopefully let you know if you like the testing style before really commiting to it.

## Setup for your local Rust project
In your rust project:

### Prerequisites
- Install Ruby, preferably the same version as the one used here
- Add a gemfile containing `gem 'cucumber'`, similar to the [example](Gemfile)
  - Note: The examples don't have a Gemfile because they piggyback off of the gemfile in the main project

### Project configuration
- Add the cucumber dependency to your Cargo.toml
- Add a feature directory, containing the following basic files
  - features/cuke.rs (based off the [example](examples/calculator/features/cuke.rs))
  - features/step_definitions/cucumber_rust.wire (based off the [example](examples/calculator/features/step_definitions/cucumber_rust.wire))
- Add a cargo test entry to your Cargo.toml to hook cucumber tests into `cargo test`(based off [example](examples/calculator/Cargo.toml))

### Usage
- Run cargo test, and verify that cucumber executes with no steps
- Add your `*.feature` files and step defintions as normal

## Running the Cucumber tests for Cucumber
Just `cargo test`. Its configured (as in the examples) to be run with the rest of the tests.

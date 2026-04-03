# Contributing to Faker-Rust

We are always happy to make improvements to Faker-Rust. There are many ways to contribute, from writing tutorials or blog posts, improving the documentation, submitting bug reports and feature requests, or writing code that can be incorporated into Faker-Rust itself.

Following these guidelines helps communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect by addressing your issue, assessing changes, and helping you finalize your pull requests.

## Reporting a bug

Have a fix for a problem you've been running into? Here's what you need to do:

- Fork this repo and clone your fork to somewhere on your machine.
- [Ensure that you have a working environment](#setting-up-your-environment).
- Read up on the [architecture](#architecture), [how to run tests](#running-the-tests), and [the code style we use in this project](#code-style).
- Cut a new branch and write a failing test for the feature or bugfix you plan on implementing.
- [Make sure your branch is well managed as you go along](#managing-your-branch).
- [Refrain from updating the changelog](#a-word-on-the-changelog).
- Push to your fork and submit a pull request.
- [Ensure that the test suite passes on GitHub Actions and make any necessary changes to your branch to bring it to green](#continuous-integration).

## What contributions we are looking for

Faker-Rust is a port of the [Ruby Faker gem](https://github.com/faker-ruby/faker). We are looking for contributions that help us achieve full parity with the original gem, as well as:

- adding new translations
- updating the existing translations
- fixing any outdated/wrong translations
- removing harmful/offensive words from locales
- implementing missing generators from the Ruby version

Although we maintain Faker-Rust in our free time, we try to respond to contributions in a timely manner. Once we look at your pull request, we may give you feedback. For instance, we may suggest some changes to make to your code to fit within the project style or discuss alternate ways of addressing the issue in question. Assuming we're happy with everything, we'll then bring your changes into main. Now you're a contributor!

## Setting up your environment

Faker-Rust requires a stable version of Rust (>= 1.70 recommended). After forking and cloning the repo, navigate to the directory and run:

```bash
cargo build
```

Run `cargo test` to ensure the project is all set up. It runs the tests and ensures dependencies are correctly resolved.

## Architecture

This project follows the typical structure for a Rust crate: code is located in `src/`. Generators and logic are organized into modules. Locale data is stored in the `locales/` directory at the root of the project.

## Running the tests

To run all of the tests, simply run:

```bash
cargo test
```

## Code Style

We use `rustfmt` and `clippy` to maintain code quality and consistency.

Please follow these guidelines when adding new code:
* Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types).
* Use four spaces for indentation (standard Rust style).
* No trailing whitespace.
* In general, follow the conventions you see being used in the source already.
* Clippy warnings should be resolved before a PR is approved.
  * To fix common offenses automatically, run `cargo clippy --fix --allow-dirty`.

There are a few ways to check code quality:

```bash
cargo clippy    # Run Clippy linter
cargo fmt --all # Format all files in the crate
```

## Managing your branch

- Use well-crafted commit messages and Pull Request descriptions, providing context where possible.
- When updating documentation or the README, you can skip running CI by adding `[skip ci]` to your commit message.
- Squash "WIP" commits and remove merge commits by rebasing your branch against `main`. We try to keep our commit history as clean as possible.

## Adding new generators/locales

We are actively looking to achieve 100% parity with the Ruby Faker gem. If you find a generator or locale missing, feel free to implement it!

### General Guidelines

Before opening a PR for a new generator, please review these guidelines:

- Avoid:
  - Hurtful language that can convey exclusionary behavior, such as racism, sexism, homophobia.
  - Graphically violent or harmful terms towards any living beings.
  - Unnecessarily gendered language.
- Be considerate and mindful of others.
- When adding new generators, limit the number of values per generator in the YAML file to keep it manageable.
- Don't use standard library random functions directly (like `rand::random`). Instead, use the RNG provided by `FakerConfig::current().rng`. This ensures that the deterministic seeding feature remains functional.
- Make sure the generator doesn't exist already before opening a PR.
- Add new YAML files to the `locales/` directory.

### Documentation

Add the new generator to the [Generators list in the README](./README.md#generators) so others can find it.

#### Rustdoc

Include standard Rust documentation comments (`///`) for all public methods:
- A short description of what the method generates.
- Documentation for parameters and return types.
- At least one doc test/example of the output.

Example:

```rust
/// Produces a random string of alphabetic characters (no digits)
///
/// # Arguments
///
/// * `number` - The length of the string to generate
///
/// # Example
///
/// ```rust
/// use faker_rust::alphanumeric;
/// let result = alphanumeric::alpha(Some(10));
/// assert_eq!(result.len(), 10);
/// ```
pub fn alpha(number: Option<usize>) -> String {
    // ...
}
```

## Removing/Deprecating Generators

To remove a generator or any other public method, please deprecate it first. In Rust, we use the `#[deprecated]` attribute.

Example:

```rust
#[deprecated(since = "0.2.0", note = "please use `Faker::new_method` instead")]
pub fn old_method() {
    // ...
}
```

We recommend adding tests to ensure that the deprecated methods still work as expected until they are fully removed in a major release.

## YAML files

Please use dash syntax for YAML arrays. The dash syntax facilitates code reviews by making it easier to see what items were added or removed from the lists.

Example:

```yaml
# this is preferred
a_things:
  - small_thing
  - big_thing
  - other_thing

# instead of these
b_things: [small_thing, big_thing, other_thing]
c_things: [
  small_thing,
  big_thing,
  other_thing,
]
```

## Tips

* Use `cargo run --example cli_demo` to see the CLI in action.
* Use `cargo doc --open` to build and view the documentation in your browser.

## A word on the Changelog

You may also notice that we have a changelog in the form of `CHANGELOG.md`. Please do not update this file in your PR — we'll take care of it during the release process!

## Continuous integration

GitHub Actions will run automatically after you push a branch or open a PR. It takes a few minutes to run the test suite. If the build fails, click on the failed job to see the output and address any issues.

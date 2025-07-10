# Contributing to cruct

In cruct, we welcome contributions from everyone, including bug reports,
pull requests, and feedback. This document serves as guidance if you are
considering submitting any of the above.

## Submitting Bug Reports and Feature Requests

To submit a bug report or feature request, you can open an issue in this
repository: [`FlakySL/cruct`].

When reporting a bug or requesting help, please include sufficient details
to allow others to reproduce the behavior you're encountering. For guidance on
how to approach this, read about [How to Create a Minimal, Reproducible Example].

When making a feature request, please clearly explain:

1. The problem you want to solve
2. How cruct could help address this problem
3. Any potential alternatives
4. Possible disadvantages of your proposal

Before submitting, please verify that no existing issue addresses your specific
problem/request. If you want to elaborate on a problem or discuss it further,
you can use our [Discord channel] at Flaky.

We recommend using the issue templates provided in this repository.

## Making Pull Requests

Before adding a feature on your behalf, we'd rather for it to be evaluated
in a issue before, we appreciate the time and effort our contributors have
and we don't want to waste it, so we'd rather talk about your feature before
you working on it.

When submitting a pull request make sure the code you added is tested and
documented, if it isn't you will be asked to document/test it before merging.

To add tests please refer to the [testing documentation] in the root folder.

## Running Tests and Compiling the Project

This project doesn't have any specific configuration
to it, meaning you can simply use `cargo build` to compile
the project and `cargo test` to test it.

The makefile in the project is used only in the workflow
runs and provides coverage for `codecov` and other statistics.

## Code of Conduct

The Flaky community follows the [Contributor covenant].
For moderation issues or escalation, please contact Esteve or Luis at
[moderation@flaky.es] rather than the Rust
moderation team.

[testing documentation]: ./TESTING.md
[Contributor covenant]: ./CODE_OF_CONDUCT.md

[`FlakySL/cruct`]: https://github.com/FlakySL/cruct

[Discord channel]: https://discord.gg/AJWFyps23a
[moderation@flaky.es]: mailto:moderation@flaky.es

[How to Create a Minimal, Reproducible Example]: https://stackoverflow.com/help/minimal-reproducible-example

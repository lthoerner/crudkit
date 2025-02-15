> *Written by Lowell Thoerner*

# Contributing to CRUDkit
Thanks for taking the time to check out CRUDkit! If you've gotten this far, you may be thinking
about contributing some code to the project.

## Beginner Contributions
We appreciate all contributions, whether small or large. We want to make sure that this project and
its community create a welcoming environment for beginner contributors. Note that when we say
"beginners," this has a variety of meanings. Some prospective contributors may be entirely new to
Rust. Some may be new to open-source development. Many may be experienced in both respects but
simply unfamiliar with the CRUDkit project itself. Below is some guidance for people who fall into
one or more of these categories. Beyond this advice, I am personally happy to provide some
mentorship for you if this document leaves you feeling lost or otherwise confused. Contact me on
Discord @eyesonjune if you need help.

### Specific Advice

#### For Beginner Project Contributors
The file structure of this project is relatively simple, and we have tried to keep the modules
highly organized to avoid overwhelmingly large and complex files. However, there is some complexity
to the architecture of the project.

The heart of this project is a set of traits that are implemented on types representing records (AKA
rows) and relations (AKA tables and views). When implemented, these traits provide the user with a
set of handler functions to perform basic operations on the database through an Axum route. They can
also be used to do operations on the database directly, though their ergonomics are not as
well-suited to this purpose.

By and large, the aforementioned traits are not designed to be implemented manually. Instead, they
use a mix of provided implementations and derive macros to generate trait implementations from a
variety of user-provided metadata.

#### For Beginner Open-Source Contributors
Open-source development is not a technically-complex process, but it is a socially-complex one. The
general mechanism for contributing to an open-source project is a "pull request," which is
essentially the process of providing the maintainers of the project with your proposed changes,
which they can then review and decide whether to accept or reject.

To begin, you must create a fork of this repository. GitHub makes it quite easy to create a fork of
a repository (there's a button near the top-right of the repository page if you are logged in). Once
you have created your fork, you can push your commits to your fork, then navigate to the GitHub page
for the forked repository and create a pull request using the prompts it provides you.

You will need some familiarity with Git in order to contribute to this project. Git is intimidating
and often clunky but it is overall a good tool and is not very difficult to use once you use it for
a little while.

#### For Beginner Rust Developers
If you are a beginner to Rust coming from another language, it's recommended you get at least a
baseline foundation on Rust syntax and conventions. If you find that you learn best from written
materials, the ["Rust book"](https://doc.rust-lang.org/book/) (*The Rust Programming Language* by
Steve Klabnik and Carol Nichols) is a great place to start. If you learn more by trial and error,
it's recommended that you join the
[Rust Programming Language Community Server](https://discord.gg/rust-lang-community) on Discord to
chat with other Rust developers about projects you're working on, and to get help when you need it.

The specific areas you should focus on for use in this project are going to be
[traits](https://doc.rust-lang.org/book/ch10-02-traits.html) and
[derive macros](https://doc.rust-lang.org/book/ch19-06-macros.html). These are covered in the Rust
book, but not very in-depth. There are a few good video resources on the subject, and in regard to
derive macros, I personally learned from these videos:
- [*A Practical Introduction to Derive Macros in Rust* - Schrödinger's Watermelon](https://youtu.be/XY0yR6IPbhw)
- [*A Practical Introduction to Derive Macros with Attributes* - Schrödinger's Watermelon](https://youtu.be/GFijwucFJqw)

If you are entirely new to programming, we absolutely welcome you to involve yourself in discussions
about the project if you would like. That being said, this is most likely not a good first project
to work on due to the complexity both in its implementation and in its use case.

### General Advice

> *To be expanded upon later.*

#### Issue Difficulty
In the process of developing this project, there will inevitably arise issues of varying degrees of
complexity. We will try our best to make sure that issues are marked with "expected difficulty" tags
so that contributors like you can easily jump in and find something to work on.

## Code Conventions \& Quality
Generally speaking, the code that you contribute should roughly match the style of the code around
it. This is not a strict rule by any means, and most of the time maintainers will provide minor
revisions when code is submitted in a PR.

Here are some general guidelines:
- All modules should use the `mod.rs` module root format.
- Module declarations should be placed at the top of a file, above all imports.
- Import paths should be fully clarified and should not rely on any more than a single layer of
re-exports.
- Imports should not be nested.
- Imports should be separated into three sections: standard library imports, external crate imports,
and internal imports.
- Internal imports should use the full `crate::` path if more than one `super::` would be needed.
- Re-exports should not be used internally.
- File-wide attributes such as `#![allow(...)]` should not be used.
- Functions should not exceed a few dozen lines when avoidable.
- Variables and type names should not be abbreviated in any way, except in the case of universal
conventions such as iterator variables and single-letter generic type parameter names.
- Comments should use proper grammar and syntax.
- Comments should not be used excessively, but should be descriptive and thorough when necessary.
- All public-facing items should be documented using doc comments.

## Editor Setup
> *To be expanded upon later.*

### Extensions
> *To be expanded upon later.*

### Settings
> *To be expanded upon later.*

## Commit Etiquette
> *To be expanded upon later.*

## Pull Requests
> *To be expanded upon later.*

## Testing Workflows
> *To be expanded upon later.*

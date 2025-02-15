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

### Guidelines
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
It is highly recommended that you use Visual Studio Code to work on this project. If you wish to use
another editor you are absolutely welcome to, but we do not officially support any other editors in
terms of publishing setup guides or scripts.

Though the aesthetic customization of your editor is entirely up to you, there are some extensions
and settings that we suggest you use. These are detailed below.

### Extensions
#### Language Support Tools
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer): adds
Rust LSP support
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml):
adds TOML LSP support
- [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax):
improves syntax highlighting for Rust
#### Formatting Tools
- [Reflow Markdown](https://marketplace.visualstudio.com/items?itemName=marvhen.reflow-markdown):
automatically reflows Markdown file contents to fit a specified width
#### Visualization Tools
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens): displays
errors inline rather than having to hover over them with a cursor or use `cargo check`
- [Better Comments](https://marketplace.visualstudio.com/items?itemName=aaron-bond.better-comments):
adds color highlighting for comments that can visually indicate the purpose of the comment more
conveniently
#### Collaboration Tools
-
  [GitHub Pull Requests](https://marketplace.visualstudio.com/items?itemName=GitHub.vscode-pull-request-github):
  adds a pull request browser sidebar for managing your own and others' pull requests

### Settings
> *To be expanded upon later.*

## Collaboration

### Commit Etiquette
#### Commit Messages
CRUDkit uses a simplified version of the
["Conventional Commits"](https://www.conventionalcommits.org/en/v1.0.0/) format. Please ensure that
your commits include a commit message. At the moment, we do not feel the project codebase is large
enough to include scope qualifiers in the messages. Try to avoid using commit types other than
`feat`, `fix`, or `refactor`. Once an initial release has been published, we will require the `!`
breaking change specifier to be added to messages, as in `feat!`.

Do note that if your commits do not have messages or are improperly formatted, they will be edited
by maintainers, which can cause issues when you try to sync your local Git repository with the
upstream. If your commits do not have messages and their purpose is not easily discernible from the
diff, a maintainer will ask you for more information.

Commit should not include a description. If a commit is complex enough to warrant a description, it
is likely too large and should be split up into separate commits.

#### Commit Frequency
Commits should not be extraordinarily large. There are some exceptions to this, such as when moving
large codeblocks between files, where there will be many lines changed but minimal real changes to
the codebase. Extremely small commits are completely fine, but we recommend that you commit at the
point where you feel like you can write a meaningful commit message. If you are just trying to keep
a thorough history while you work on a feature, you can make commits with no messages in your fork
and then squash them into one larger commit with a message later.

### Issues
Issues are the main way of discussing bugs, feature requests, and other ideas regarding the project.
We do not use GitHub's "Discussions" feature because it is too similar in usage to Issues.

#### Writing Issues
Please be thorough when writing an issue. Use proper syntax and grammar. If you are not a native
English speaker, there are a lot of tools that can help revise your writing, but it is not a major
problem if your issue has some improper or odd phrasing. All we require is that you make your best
effort, and maintainers will ask for clarification if necessary.

If the issue you are writing pertains to a bug, please try your best to find a way to reproduce it
and give examples of how others could encounter the bug. If the issue pertains to an idea or feature
request, please try to give some examples of how you think the feature could be used, preferably
including a few code snippets demonstrating how the user-facing API might work.

#### Duplicate Issues
We request that you look for similar issues prior to opening a new one, but if it appears that an
issue has been abandoned, you may open a new issue so long as it mentions the original issue(s),
preferably with links. If it is deemed that your issue is a duplicate, a maintainer will comment on
it with information about what other issue(s) they believe it is a duplicate of. There will be a
two-day period for you to respond, either to confirm that it is a duplicate, or to dispute it and
clarify the issue as needed. If you do not respond within this time, the issue will be marked as a
duplicate and archived.

### Pull Requests
Pull requests (PRs) are the main way to contribute to CRUDkit, and indeed essentially any
open-source project. The way we do PRs is very typical, so if you are an experienced open-source
developer you should feel right at home.

#### Guidelines
- An issue should be filed prior to making a PR. This is not for the maintainers' sake but for your
own. Filing an issue will ensure that your contribution is not a duplicate and does not conflict
with other in-progress contributions, so that you do not waste time doing a bunch of work only to
find out someone else is working on it already, or that it does not fit into the project's design.
- For contributions which are approved via an issue but still being worked on, draft PRs should be
used until the contribution is ready for review.
- Once a PR is submitted and ready for review, you may message the maintainers either through a
mention in the comments of the PR or through Discord. It should go without mentioning that you are
not to spam or harass anyone in this process.
- Keep up-to-date with your own PR and be responsive with maintainers and other contributors as they
make suggestions or edits. If you do not respond for some time, maintainers may make the edits and
merge the PR without further consulting you.

#### Merging/Rebasing
When a PR has been reviewed and is deemed ready to merge, we will merge it into `main` or an
appropriate development branch. We generally prefer to use rebase rather than merge in order to keep
an easily-navigable linear history. If this eventually becomes untenable we may switch to using
merges. Colloquially, we use the term "merge" to mean "merge or rebase," so don't worry about the
nomenclature.

#### Ignored Files
Please do not commit your `.vscode` directory or any other editor, extension, or tool configuration
to the Git repository. Do not edit the `.gitignore` unless there is a compelling reason to do so.
Instead, use `.git/info/exclude` for a local-only blacklist.

### Testing Workflows
At the moment we do not have any testing workflows, but this will change in the near future.
Specifically, we will be setting up regression tests using Cargo's built-in testing framework and we
will request that contributors ensure their code passes the test suite before publishing a final PR.

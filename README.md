> *Written by Lowell Thoerner*

# CRUDkit
CRUDkit is a Rust crate that aims to provide a convenient framework to build CRUD application
backends using [Axum](https://crates.io/crates/axum) and PostgreSQL, acting as an abstraction over
[SQLX](https://crates.io/crates/sqlx). In the future, it will likely be expanded to work with MySQL
and SQLite as well.

CRUDkit lives somewhere in between a query builder and an ORM. It does not provide granular
low-level control of SQL queries, nor does it aim to provide a type-safe implementation of SQL. It
does not aim to automate the process of creating a database schema, nor the process of creating a
backend to match said schema. It simply provides helpful interfaces to allow the two systems to
communicate with each other using shared data structures.

## What CRUDkit Does
- Generate handler functions for typical database operations (CREATE, READ, UPDATE, DELETE)
- Allow for smooth interoperation between backend types and data in a relational database
- Remove the need for writing massive amounts of boilerplate query-building logic
- Provide some metadata about database types for use in custom logic, such as a list of column names

## What CRUDkit Doesn't Do
- Generate backend types from a database schema, or vice versa
- Work with non-relational databases and unstructured data
- Provide high-level abstractions to be used in complex endpoints

> *To be expanded upon later.*

## Installing CRUDkit
Installing CRUDkit is mostly a pretty simple process, but there is one complication that you need to
take into account. Because reexporting it breaks some derive macros, SQLx must be installed in your
own crate in order to use CRUDkit. Additionally, it must be an identical version to the one used in
CRUDkit.

As usual, the first step of installing the crate is to `cargo add` it:
```sh
cargo add crudkit
```
Next, add SQLx:
> Do not add a space after the comma between features, it will not work!
```sh
cargo add sqlx@0.8.3 --features postgres,runtime-tokio
```
Lastly, add Serde:
```sh
cargo add serde
```

At this point, you should be able to use CRUDkit in your crate. All other dependencies are
re-exported from the crate so you do not have to worry about versions or features.

## Code of Conduct
Please be aware that the maintainers and other developers of this project are people too, with their
own lives, responsibilities, and circumstances. Just like in most open-source development, this is
volunteer work, and contributors should not be holding each other to the same standards they would
expect of a coworker in terms of productivity. However, all contributors (including maintainers) are
expected to follow the same general rules of etiquette that would be found in the average workplace.
Toxicity will absolutely not be tolerated from anyone, regardless of the magnitude of their
contribution. On a personal note, I expect you to hold me to this standard, and if I do not live up
to it, it is well within your right under the license of this software to create a fork.

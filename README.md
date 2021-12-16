[![get diff and lint](https://github.com/alicika/grepcli/actions/workflows/main.yml/badge.svg)](https://github.com/alicika/grepcli/actions/workflows/main.yml)

# grepcli
Extended and Fortified version of the default grep mock. <br>
Provides the better error message than default, with use of `thiserror` crate. <br>
Achieves separation by concerns with clear devision of codes by their purpose. <br>
Persist code quality by testing and enabling to Lint/format by `Clippy` and `rustfmt` through CI with GitHub Actions.

# Manual Build
`$ cargo build `

# Requirements
See [Cargo.toml](/Cargo.toml). <br>
Can choose the third-party crate dependent stuff (on main branch), or the independent stdlib thing (on stdlib branch).

# Usage
` grepcli <query> <path/to/file> `
If missing either or where there's a problem, the shell shows a precise, context-affected error by which the problem was caused.

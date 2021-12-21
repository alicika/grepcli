[![get diff and lint](https://github.com/alicika/grepcli/actions/workflows/main.yml/badge.svg)](https://github.com/alicika/grepcli/actions/workflows/main.yml)

# grepcli
Extended and enhanced version of the grep mock.  
Provides context-based error messages than using the standard library as-is.  
Achieves separation by concerns with clear devision of codes with their purpose.  
Persist code quality by testing and enabling to Lint/format by `Clippy` and `rustfmt` through CI with GitHub Actions.  

# Manual Build
```bash
$ cargo build --release`
```  

# Requirements
See [Cargo.toml](/Cargo.toml).  
Can choose the third-party crate dependent stuff (on main branch), or the independent stdlib thing (on stdlib branch).

# Usage
```bash
 grepcli <query> <path/to/file> 
 ```  
 
If missing either or where there's a problem, the shell shows a precise, context-based error by which the problem was caused.  

# quack

travis-ci: [![Build Status](https://travis-ci.org/booyaa/quack.svg?branch=master)](https://travis-ci.org/booyaa/quack)

A [Duck Duck Go](https://duckduckgo.com) [InstantAnswer API](https://duckduckgo.com/api) library written in rust.

# Documentation

- [doc.rs](https://docs.rs/quack) ![badge](https://docs.rs/quack/badge.svg)
- [github fallback](https://booyaa.github.io/quack/quack/index.html)

# Usage

This crate is on [crates.io](https://crates.io/crates/quack) and can be
used by adding `quack` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
quack = "0.1"
```

and this to your crate root:
```
extern crate quack;
```

```rust
use quack::Quack;
println!("{:#?}", Quack::new("!imdb+rushmore"));
```

# See also

[qwk](https://github.com/booyaa/qwk) - a cli

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.

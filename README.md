# quack

travis-ci: [![Build Status](https://travis-ci.org/booyaa/quackngo.svg?branch=master)](https://travis-ci.org/booyaa/quackngo)

A [Duck Duck Go](https://duckduckgo.com) [InstantAnswer API](https://duckduckgo.com/api) library written in rust.

# Documentation

- [doc.rs](https://docs.rs/quackngo) ![badge](https://docs.rs/quackngo/badge.svg)
- [github fallback](https://booyaa.github.io/quackngo/quackngo/index.html)

# Usage

This crate is on [crates.io](https://crates.io/crates/quackngo) and can be
used by adding `quack` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
quackngo = "0.1"
```

and this to your crate root:

```
extern crate quackngo;
```

```rust
use quackngo::Quack;
println!("{:#?}", Quack::new("!imdb+rushmore"));
```

# See also

[qwk](https://github.com/booyaa/qwk) - a cli

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.

# chemfiles.rs

[![Build Status](https://travis-ci.org/chemfiles/chemfiles.rs.svg?branch=master)](https://travis-ci.org/chemfiles/chemfiles.rs)
[![codecov.io](https://codecov.io/github/chemfiles/chemfiles.rs/coverage.svg?branch=master)](https://codecov.io/github/chemfiles/chemfiles.rs?branch=master)
[![Documentation](https://img.shields.io/badge/docs-latest-brightgreen.svg)](http://chemfiles.org/chemfiles.rs/)

Rust binding for the [chemfiles](https://github.com/chemfiles/chemfiles)
library.

## [Documentation](http://chemfiles.org/chemfiles.rs/)

## Usage

Add this to your `Cargo.toml` file:

```toml
[dependencies]
chemfiles = "0.9"
```

And then use `chemfiles` in your project:

```rust
extern crate chemfiles;
```

Here is a simple usage example. See the `examples` folder for additional
examples.

```rust
extern crate chemfiles;
use chemfiles::{Trajectory, Frame};

fn main() {
    let mut trajectory = Trajectory::new("filename.xyz").unwrap();
    let mut frame = Frame::new(0).unwrap();

    trajectory.read(&mut frame).unwrap();

    println!("There are {} atoms in the frame", frame.natoms().unwrap())

    let positions = frame.positions().unwrap();

    // Do awesome things with the positions here !
}
```

## Bug reports, feature requests

Please report any bug you find and any feature you may want as a Github [issue].

[issue]: https://github.com/chemfiles/chemfiles.rs/issues/new

# Chemharp.rs

Rust binding for the [Chemharp](https://github.com/Luthaf/Chemharp) library.

## Installation

First, build and install Chemharp. Then, it is as easy as:

```
git clone https://github.com/Luthaf/Chemharp.rs
cd Chemharp.rs
cargo build
```

## Usage example

Here is a simple usage example for the `chemharp` crate. Please see the `examples` folder
for other examples.

```rust
extern crate chemharp;

use chemharp::Trajectory;

fn main() {
    let mut trajectory = Trajectory::new("filename.xyz").unwrap();
    let mut frame = Frame::new(0).unwrap();

    trajectory.read(&mut frame).unwrap();

    println!("There are {} atoms in the frame", frame.natoms().unwrap())

    let positions = frame.positions().unwrap();

    // Do awesome things with the positions here !
}
```

See the `examples` folder for other examples.

## Bug reports, feature requests

Please report any bug you find and any feature you may want as a [github issue](https://github.com/Luthaf/Chemharp.rs/issues/new).

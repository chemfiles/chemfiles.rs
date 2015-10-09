# chemfiles.rs

Rust binding for the [chemfiles](https://github.com/chemfiles/chemfiles) library.

## Installation

First, build and install chemfiles. Then, it is as easy as:

```
git clone https://github.com/chemfiles/chemfiles.rs
cd chemfiles.rs
cargo build
```

## Usage example

Here is a simple usage example for the `chemfiles` crate. Please see the `examples` folder
for other examples.

```rust
extern crate chemfiles;
use chemfiles::Trajectory;

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

Please report any bug you find and any feature you may want as a [github issue](https://github.com/chemfiles/chemfiles.rs/issues/new).

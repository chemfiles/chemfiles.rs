/* This file is an example for the chemfiles library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
use chemfiles::{Frame, Selection, Trajectory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Trajectory::open("input.arc", 'r')?;
    let mut output = Trajectory::open("output.pdb", 'w')?;

    let mut selection = Selection::new("name Zn or name N")?;

    let mut frame = Frame::new();
    for _ in 0..input.nsteps() {
        input.read(&mut frame)?;

        let mut to_remove = selection.list(&frame);
        to_remove.sort_unstable();
        to_remove.reverse();
        for i in to_remove {
            frame.remove(i);
        }

        output.write(&frame)?;
    }

    Ok(())
}

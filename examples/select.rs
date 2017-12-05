/* This file is an example for the chemfiles library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
extern crate chemfiles;
use chemfiles::{Frame, Selection, Trajectory};

fn main() {
    let mut input = Trajectory::open("input.arc", 'r').unwrap();
    let mut output = Trajectory::open("output.pdb", 'w').unwrap();

    let mut selection = Selection::new("name Zn or name N").unwrap();

    let mut frame = Frame::new().unwrap();
    for _ in 0..input.nsteps().unwrap() {
        input.read(&mut frame).unwrap();

        let mut to_remove = selection.list(&frame).unwrap();
        to_remove.sort();
        to_remove.reverse();
        for i in to_remove {
            frame.remove(i as usize).unwrap();
        }

        output.write(&frame).unwrap();
    }
}

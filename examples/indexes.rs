/* This file is an example for the chemfiles library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
use chemfiles::{Frame, Trajectory};

fn main() {
    let mut file = Trajectory::open("filename.xyz", 'r').unwrap();
    let mut frame = Frame::new();
    file.read(&mut frame).unwrap();

    let mut less_than_five = vec![];
    for (i, position) in frame.positions().iter().enumerate() {
        if position[0] < 5.0 {
            less_than_five.push(i);
        }
    }

    println!("Atoms with x < 5: ");
    for i in less_than_five {
        println!("  - {}", i);
    }
}

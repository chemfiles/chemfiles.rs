// File indexes.rs, example for the chemfiles library
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
extern crate chemfiles;
use chemfiles::*;

fn main() {
    let mut traj = Trajectory::open("filename.xyz", 'r').unwrap();
    let mut frame = Frame::new().unwrap();

    traj.read(&mut frame).unwrap();
    let positions = frame.positions().unwrap();
    let mut indexes = Vec::new();

    for (i, position) in positions.iter().enumerate() {
        if position[0] < 5.0 {
            indexes.push(i);
        }
    }

    println!("Atoms with x < 5: ");
    for i in indexes {
        println!(" - {}", i);
    }
}

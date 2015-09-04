/* File indexes.rs, example for the Chemharp library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
extern crate chemharp;
use chemharp::*;

fn main() {
    let mut traj = Trajectory::open("filename.xyz").unwrap();
    let mut frame = Frame::new(0).unwrap();

    traj.read(&mut frame).unwrap();
    let positions = frame.positions().unwrap();
    let mut indexes = Vec::new();

    for i in 0..frame.natoms().unwrap() {
        if positions[i][0] < 5.0 {
            indexes.push(i);
        }
    }

    println!("Atoms with x < 5: ");
    for i in indexes.iter() {
        println!(" - {}", i);
    }
}

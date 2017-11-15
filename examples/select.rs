// File select.rs, example for the chemfiles library
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
extern crate chemfiles;
use chemfiles::{Frame, Selection, Trajectory};

fn main() {
    // Read a frame from a given file
    let mut trajectory = Trajectory::open("filename.xyz", 'r').unwrap();
    let mut frame = Frame::new().unwrap();
    trajectory.read(&mut frame).unwrap();

    // Create a selection for all atoms with "Zn" name
    let mut selection = Selection::new("name Zn").unwrap();
    // Get the list of matching atoms from the frame
    let zincs = selection.list(&frame).unwrap();

    println!("We have {} zinc in the frame", zincs.len());
    for i in zincs {
        println!("{} is a zinc", i);
    }

    // Create a selection for multiple atoms
    let mut selection = Selection::new("angles: name(#1) H and name(#2) O and name(#3) H").unwrap();
    // Get the list of matching atoms in the frame
    let waters = selection.evaluate(&frame).unwrap();

    println!("We have {} water molecules", waters.len());
    for water in waters {
        println!("{} - {} - {} is a water", water[0], water[1], water[2]);
    }
}

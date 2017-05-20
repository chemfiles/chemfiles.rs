// File convert.rs, example for the chemfiles library
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
extern crate chemfiles;
use chemfiles::{Trajectory, Frame, Topology,  UnitCell, Atom};

fn main() {
    let mut input = Trajectory::open("water.xyz", 'r').unwrap();
    let mut frame = Frame::new().unwrap();
    let mut topology = Topology::new().unwrap();
    // Orthorombic UnitCell with lengths of 20, 15 and 35 A
    let cell = UnitCell::new(20.0, 15.0, 35.0).unwrap();

    // Create Atoms
    let oxygen = Atom::new("O").unwrap();
    let hydrogen = Atom::new("H").unwrap();

    // Fill the topology with one water molecule
    topology.add_atom(&oxygen).unwrap();
    topology.add_atom(&hydrogen).unwrap();
    topology.add_atom(&hydrogen).unwrap();
    topology.add_bond(0, 1).unwrap();
    topology.add_bond(0, 2).unwrap();

    // Set the frame cell and topology for all the trajectory steps
    input.set_cell(&cell).unwrap();
    input.set_topology(&topology).unwrap();

    let mut output = Trajectory::open("water.pdb", 'w').unwrap();

    for _ in 0..input.nsteps().unwrap() {
        input.read(&mut frame).unwrap();
        // Write the frame to the output file, using PDB format
        output.write(&frame).unwrap();
    }
}

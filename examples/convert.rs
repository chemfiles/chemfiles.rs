/* File convert.rs, example for the chemfiles library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
extern crate chemfiles;
use chemfiles::*;

fn main() {
    let mut input = Trajectory::open("water.xyz").unwrap();
    let mut frame = Frame::new(0).unwrap();
    let mut water_topology = Topology::new().unwrap();
    // Orthorombic UnitCell with lengths of 20, 15 and 35 A
    let cell = UnitCell::new(20.0, 15.0, 35.0).unwrap();

    // Create Atoms
    let atom_o = Atom::new("O").unwrap();
    let atom_h = Atom::new("H").unwrap();

    // Fill the topology with one water molecule
    water_topology.push(&atom_o).unwrap();
    water_topology.push(&atom_h).unwrap();
    water_topology.push(&atom_h).unwrap();
    water_topology.add_bond(0, 1).unwrap();
    water_topology.add_bond(0, 2).unwrap();

    let mut output = Trajectory::create("water.pdb").unwrap();

    for _ in 0..input.nsteps().unwrap() {
        input.read(&mut frame).unwrap();
        // Set the frame cell and topology
        frame.set_cell(&cell).unwrap();
        frame.set_topology(&water_topology).unwrap();
        // Write the frame to the output file, using PDB format
        output.write(&frame).unwrap();
    }
}

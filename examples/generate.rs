/* This file is an example for the chemfiles library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
extern crate chemfiles;
use chemfiles::{Atom, Frame, Topology, Trajectory, UnitCell};

fn main() {
    let mut topology = Topology::new().unwrap();
    topology.add_atom(&Atom::new("H").unwrap()).unwrap();
    topology.add_atom(&Atom::new("O").unwrap()).unwrap();
    topology.add_atom(&Atom::new("H").unwrap()).unwrap();

    topology.add_bond(0, 1).unwrap();
    topology.add_bond(2, 1).unwrap();

    let mut frame = Frame::new().unwrap();
    frame.resize(3).unwrap();

    {
        let positions = frame.positions_mut().unwrap();
        positions[0] = [1.0, 0.0, 0.0];
        positions[1] = [0.0, 0.0, 0.0];
        positions[2] = [0.0, 1.0, 0.0];
    }

    frame.add_atom(&Atom::new("O").unwrap(), [5.0, 0.0, 0.0], None).unwrap();
    frame.add_atom(&Atom::new("C").unwrap(), [6.0, 0.0, 0.0], None).unwrap();
    frame.add_atom(&Atom::new("O").unwrap(), [7.0, 0.0, 0.0], None).unwrap();
    frame.add_bond(3, 4).unwrap();
    frame.add_bond(4, 5).unwrap();

    frame.set_cell(&UnitCell::new([10.0, 10.0, 10.0]).unwrap()).unwrap();

    let mut trajectory = Trajectory::open("water-co2.pdb", 'w').unwrap();
    trajectory.write(&frame).unwrap();
}

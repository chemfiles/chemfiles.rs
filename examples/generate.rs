/* This file is an example for the chemfiles library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
use chemfiles::{Atom, Frame, Topology, Trajectory, UnitCell};

fn main() {
    let mut topology = Topology::new();
    topology.add_atom(&Atom::new("H"));
    topology.add_atom(&Atom::new("O"));
    topology.add_atom(&Atom::new("H"));

    topology.add_bond(0, 1);
    topology.add_bond(2, 1);

    let mut frame = Frame::new();
    frame.resize(3);
    {
        let positions = frame.positions_mut();
        positions[0] = [1.0, 0.0, 0.0];
        positions[1] = [0.0, 0.0, 0.0];
        positions[2] = [0.0, 1.0, 0.0];
    }
    frame.set_topology(&topology).unwrap();

    frame.add_atom(&Atom::new("O"), [5.0, 0.0, 0.0], None);
    frame.add_atom(&Atom::new("C"), [6.0, 0.0, 0.0], None);
    frame.add_atom(&Atom::new("O"), [7.0, 0.0, 0.0], None);
    frame.add_bond(3, 4);
    frame.add_bond(4, 5);

    frame.set_cell(&UnitCell::new([10.0, 10.0, 10.0]));

    let mut trajectory = Trajectory::open("water-co2.pdb", 'w').unwrap();
    trajectory.write(&frame).unwrap();
}

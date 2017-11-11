extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("chemfiles.h");
    cfg.include("../chemfiles-sys/chemfiles/include");
    cfg.include(".");

    cfg.skip_signededness(|s| {
        s == "chfl_warning_callback"
    });

    // ctest does not know what to do with pointers to double[N] data.
    const SKIPED_FNS: &[&str] = &[
        "chfl_topology_bonds", "chfl_topology_angles", "chfl_topology_dihedrals",
        "chfl_cell_matrix", "chfl_frame_positions", "chfl_frame_velocities",
    ];

    cfg.skip_fn(|name| {
        SKIPED_FNS.contains(&name)
    });

    cfg.generate("../chemfiles-sys/lib.rs", "ctest.rs");
}

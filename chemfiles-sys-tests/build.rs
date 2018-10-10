extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("chemfiles.h");
    cfg.include("../chemfiles-sys/chemfiles/include");
    cfg.include(".");
    cfg.type_name(|ty, _is_struct, _is_union| {
        ty.to_string()
    });

    cfg.skip_signededness(|s| s == "chfl_warning_callback" || s == "chfl_vector3d");

    // ctest does not know what to do with some pointers to pointer types
    const SKIPED_FNS: &[&str] = &[
        "chfl_topology_bonds",
        "chfl_topology_angles",
        "chfl_topology_dihedrals",
        "chfl_cell_matrix",
        "chfl_frame_positions",
        "chfl_frame_velocities",
        "chfl_topology_impropers",
        "chfl_frame_list_properties",
        "chfl_residue_list_properties",
        "chfl_atom_list_properties",
        "chfl_trajectory_path",
    ];
    cfg.skip_fn(|name| SKIPED_FNS.contains(&name));

    cfg.generate("../chemfiles-sys/lib.rs", "ctest.rs");
}

extern crate ctest2;

fn main() {
    let mut cfg = ctest2::TestGenerator::new();
    cfg.header("chemfiles.h");
    cfg.include("../chemfiles-sys/chemfiles/include");
    cfg.include(".");
    cfg.type_name(|ty, _is_struct, _is_union| ty.to_string());

    cfg.skip_roundtrip(|s| s == "chfl_vector3d" || s == "c_bool" || s == "chfl_warning_callback");

    // ctest does not know what to do with some pointers to pointer types
    const SKIPED_FNS: &[&str] = &[
        "chfl_topology_bonds",
        "chfl_topology_angles",
        "chfl_topology_dihedrals",
        "chfl_cell_matrix",
        "chfl_cell_from_matrix",
        "chfl_frame_positions",
        "chfl_frame_velocities",
        "chfl_topology_impropers",
        "chfl_frame_list_properties",
        "chfl_residue_list_properties",
        "chfl_atom_list_properties",
    ];
    cfg.skip_fn(|name| SKIPED_FNS.contains(&name));

    cfg.generate("../chemfiles-sys/lib.rs", "ctest.rs");
}

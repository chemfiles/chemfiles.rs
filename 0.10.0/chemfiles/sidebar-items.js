initSidebarItems({"enum":[["BondOrder","Possible bond order associated with bonds"],["CellShape","Available unit cell shapes."],["Property","A `Property` is a piece of data that can be associated with an `Atom` or a `Frame`."],["Status","Possible causes of error in chemfiles"]],"fn":[["add_configuration","Read configuration data from the file at `path`."],["formats_list","Get the list of formats known by chemfiles, as well as all associated metadata."],["set_warning_callback","Use `callback` for every chemfiles warning. The callback will be passed the warning message. This will drop any previous warning callback."],["version","Get the version of the chemfiles library."]],"struct":[["Atom","An `Atom` is a particle in the current `Frame`. It stores the following atomic properties:"],["AtomMut","An analog to a mutable reference to an atom (`&mut Atom`)"],["AtomRef","An analog to a reference to an atom (`&Atom`)"],["Error","Error type for Chemfiles."],["FormatMetadata","`FormatMetadata` contains metdata associated with one format."],["Frame","A `Frame` contains data from one simulation step: the current unit cell, the topology, the positions, and the velocities of the particles in the system. If some information is missing (topology or velocity or unit cell), the corresponding data is filled with a default value."],["Match","A `Match` is a set of atomic indexes matching a given selection. It can mostly be used like a `&[usize]`."],["PropertiesIter","An iterator over the properties in an atom/frame/residue"],["Residue","A `Residue` is a group of atoms belonging to the same logical unit. They can be small molecules, amino-acids in a protein, monomers in polymers, etc."],["ResidueRef","An analog to a reference to a residue (`&Residue`)"],["Selection","A `Selection` allow to select atoms in a `Frame`, from a selection language. The selection language is built by combining basic operations. Each basic operation follows the `<selector>[(<variable>)] <operator> <value>` structure, where `<operator>` is a comparison operator in `== != < <= > >=`."],["Topology","A `Topology` contains the definition of all the atoms in the system, and the liaisons between the atoms (bonds, angles, dihedrals, ...). It will also contain all the residues information if it is available."],["TopologyRef","An analog to a reference to a topology (`&Topology`)"],["Trajectory","The `Trajectory` type is the main entry point when using chemfiles. A `Trajectory` behave a bit like a file, allowing to read and/or write `Frame`."],["UnitCell","An `UnitCell` represent the box containing the atoms, and its periodicity."],["UnitCellMut","An analog to a mutable reference to an unit cell (`&mut UnitCell`)"],["UnitCellRef","An analog to a reference to an unit cell (`&UnitCell`)"]]});
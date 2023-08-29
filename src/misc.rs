// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2020 Guillaume Fraux -- BSD licensed
use std::convert::TryInto;
use std::ffi::CStr;
use std::path::Path;

use chemfiles_sys as ffi;

use crate::errors::check_success;

use crate::{errors::check, Error};

/// `FormatMetadata` contains metadata associated with one format.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatMetadata {
    /// Name of the format.
    pub name: &'static str,
    /// Extension associated with the format.
    pub extension: Option<&'static str>,
    /// Extended, user-facing description of the format.
    pub description: &'static str,
    /// URL pointing to the format definition/reference.
    pub reference: &'static str,
    /// Is reading files in this format implemented?
    pub read: bool,
    /// Is writing files in this format implemented?
    pub write: bool,
    /// Does this format support in-memory IO?
    pub memory: bool,
    /// Does this format support storing atomic positions?
    pub positions: bool,
    /// Does this format support storing atomic velocities?
    pub velocities: bool,
    /// Does this format support storing unit cell information?
    pub unit_cell: bool,
    /// Does this format support storing atom names or types?
    pub atoms: bool,
    /// Does this format support storing bonds between atoms?
    pub bonds: bool,
    /// Does this format support storing residues?
    pub residues: bool,
}

impl FormatMetadata {
    pub(crate) fn from_raw(raw: &ffi::chfl_format_metadata) -> Self {
        let str_from_ptr = |ptr| unsafe { CStr::from_ptr(ptr).to_str().expect("Invalid Rust str from C") };
        let extension = if raw.extension.is_null() {
            None
        } else {
            Some(str_from_ptr(raw.extension))
        };
        Self {
            name: str_from_ptr(raw.name),
            extension,
            description: str_from_ptr(raw.description),
            reference: str_from_ptr(raw.reference),
            read: raw.read,
            write: raw.write,
            memory: raw.memory,
            positions: raw.positions,
            velocities: raw.velocities,
            unit_cell: raw.unit_cell,
            atoms: raw.atoms,
            bonds: raw.bonds,
            residues: raw.residues,
        }
    }
}

/// Get the list of formats known by chemfiles, as well as all associated metadata.
///
/// # Example
/// ```
/// let formats = chemfiles::formats_list();
/// println!("chemfiles supports {} formats:", formats.len());
/// for format in &formats {
///     println!(
///         "   {:<15} {}",
///         format.name,
///         format.extension.as_deref().unwrap_or("")
///     );
/// }
/// ```
#[must_use]
pub fn formats_list() -> Vec<FormatMetadata> {
    let mut formats = std::ptr::null_mut();
    let mut count: u64 = 0;
    let formats_slice = unsafe {
        check_success(ffi::chfl_formats_list(&mut formats, &mut count));
        std::slice::from_raw_parts(formats, count.try_into().expect("failed to convert u64 to usize"))
    };
    let formats_vec = formats_slice.iter().map(FormatMetadata::from_raw).collect();
    unsafe {
        let _ = ffi::chfl_free(formats as *const _);
    }
    return formats_vec;
}

#[allow(clippy::doc_markdown)]
/// Get the format that chemfiles would use to read a file at the given
/// ``path``.
///
/// The format is mostly guessed from the path extension, chemfiles only tries
/// to read the file to distinguish between CIF and mmCIF files. Opening the
/// file using the returned format string might still fail. For example, it will
/// fail if the file is not actually formatted according to the guessed format;
/// or the format/compression combination is not supported (e.g. `XTC / GZ` will
/// not work since the XTC reader does not support compressed files).
///
/// The returned format is represented in a way compatible with the various
/// `Trajectory` constructors, i.e. `"<format name> [/ <compression>]"`, where
/// compression is optional.
///
/// # Errors
///
/// This function returns an error if the file format couldn't be guessed.
///
/// # Panics
///
/// This function panics if the path can't be converted to a Unicode string.
///
/// # Examples
/// ```
/// let format = chemfiles::guess_format("trajectory.xyz.xz").unwrap();
/// assert_eq!(format, "XYZ / XZ");
///
/// let format = chemfiles::guess_format("trajectory.nc").unwrap();
/// assert_eq!(format, "Amber NetCDF");
///
/// let format = chemfiles::guess_format("trajectory.unknown.format");
/// assert!(format.is_err());
/// ```
pub fn guess_format<P>(path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_str().expect("couldn't convert path to Unicode");
    let path = crate::strings::to_c(path);
    let mut buffer = vec![0; 128];
    unsafe {
        check(ffi::chfl_guess_format(
            path.as_ptr(),
            buffer.as_mut_ptr(),
            buffer.len() as u64,
        ))?;
    }
    Ok(crate::strings::from_c(buffer.as_ptr()))
}

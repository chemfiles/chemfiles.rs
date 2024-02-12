// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use chemfiles_sys as ffi;

use crate::errors::{check, check_not_null, check_success, Error, Status};
use crate::frame::Frame;
use crate::strings;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A `Match` is a set of atomic indexes matching a given selection. It can
/// mostly be used like a `&[usize]`.
pub struct Match {
    size: usize,
    atoms: [usize; 4],
}

#[allow(clippy::len_without_is_empty)]
impl Match {
    /// Get the length of the Match.
    ///
    /// # Example
    ///
    /// ```
    /// # use chemfiles::Match;
    /// let atomic_match = Match::new(&[3, 4, 5]);
    /// assert_eq!(atomic_match.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Create a new match containing the atoms in the `atoms` slice.
    ///
    /// # Panics
    ///
    /// If the slice contains more than 4 elements, which is the maximal size
    /// of a match.
    ///
    /// # Example
    ///
    /// ```
    /// # use chemfiles::Match;
    /// let atomic_match = Match::new(&[3, 4, 5]);
    /// assert_eq!(atomic_match.len(), 3);
    /// assert_eq!(atomic_match[0], 3);
    /// assert_eq!(atomic_match[1], 4);
    /// assert_eq!(atomic_match[2], 5);
    /// ```
    pub fn new(atoms: &[usize]) -> Match {
        assert!(atoms.len() <= 4);
        let size = atoms.len();
        let mut matches = [usize::max_value(); 4];
        for (i, atom) in atoms.iter().enumerate() {
            matches[i] = *atom;
        }
        Match { size, atoms: matches }
    }

    /// Iterate over the atomic indexes in the match.
    ///
    /// # Example
    ///
    /// ```
    /// # use chemfiles::Match;
    /// let atomic_match = Match::new(&[3, 4, 5]);
    /// let mut iter = atomic_match.iter();
    ///
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.atoms[..self.len()].iter()
    }
}

impl std::ops::Index<usize> for Match {
    type Output = usize;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.len());
        &self.atoms[i]
    }
}

impl<'a> IntoIterator for &'a Match {
    type Item = &'a usize;
    type IntoIter = std::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.atoms[..self.len()].iter()
    }
}

/// A `Selection` allow to select atoms in a `Frame`, from a selection
/// language. The selection language is built by combining basic operations.
/// Each basic operation follows the `<selector>[(<variable>)] <operator>
/// <value>` structure, where `<operator>` is a comparison operator in
/// `== != < <= > >=`.
#[derive(Debug)]
pub struct Selection {
    handle: *mut ffi::CHFL_SELECTION,
}

impl Clone for Selection {
    fn clone(&self) -> Selection {
        unsafe {
            let new_handle = ffi::chfl_selection_copy(self.as_ptr());
            Selection::from_ptr(new_handle)
        }
    }
}

impl Drop for Selection {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::chfl_free(self.as_ptr().cast());
        }
    }
}

impl Selection {
    /// Create a `Selection` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::CHFL_SELECTION) -> Selection {
        check_not_null(ptr);
        Selection { handle: ptr }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::CHFL_SELECTION {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::CHFL_SELECTION {
        self.handle
    }

    /// Create a new selection from the given selection string.
    ///
    /// # Errors
    ///
    /// This function fails if the selection string is invalid.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Selection;
    /// let selection = Selection::new("pairs: name(#1) H and name(#2) O").unwrap();
    /// ```
    pub fn new<'a, S: Into<&'a str>>(selection: S) -> Result<Selection, Error> {
        let buffer = strings::to_c(selection.into());
        unsafe {
            let handle = ffi::chfl_selection(buffer.as_ptr());
            if handle.is_null() {
                Err(Error {
                    status: Status::SelectionError,
                    message: Error::last_error(),
                })
            } else {
                Ok(Selection::from_ptr(handle))
            }
        }
    }

    /// Get the size of the selection, i.e. the number of atoms we are selecting
    /// together.
    ///
    /// This value is 1 for the 'atom' context, 2 for the 'pair' and 'bond'
    /// context, 3 for the 'three' and 'angles' context and 4 for the 'four'
    /// and 'dihedral' context.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Selection;
    /// let selection = Selection::new("pairs: name(#1) H and name(#2) O").unwrap();
    /// assert_eq!(selection.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        let mut size = 0;
        unsafe {
            check_success(ffi::chfl_selection_size(self.as_ptr(), &mut size));
        }
        #[allow(clippy::cast_possible_truncation)]
        return size as usize;
    }

    /// Get the selection string used to create this selection.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Selection;
    /// let selection = Selection::new("name H").unwrap();
    /// assert_eq!(selection.string(), "name H");
    /// ```
    pub fn string(&self) -> String {
        let get_string = |ptr, len| unsafe { ffi::chfl_selection_string(self.as_ptr(), ptr, len) };
        let selection = strings::call_autogrow_buffer(1024, get_string).expect("failed to get selection string");
        return strings::from_c(selection.as_ptr());
    }

    /// Evaluate a selection for a given frame, and return the corresponding
    /// matches.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Selection, Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("H"), [1.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("O"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("H"), [-1.0, 0.0, 0.0], None);
    ///
    /// let mut selection = Selection::new("pairs: name(#1) H and name(#2) O").unwrap();
    /// let matches = selection.evaluate(&frame);
    ///
    /// assert_eq!(matches.len(), 2);
    ///
    /// assert_eq!(matches[0].len(), 2);
    /// assert_eq!(matches[0][0], 0);
    /// assert_eq!(matches[0][1], 1);
    ///
    /// assert_eq!(matches[1].len(), 2);
    /// assert_eq!(matches[1][0], 2);
    /// assert_eq!(matches[1][1], 1);
    /// ```
    pub fn evaluate(&mut self, frame: &Frame) -> Vec<Match> {
        #![allow(clippy::cast_possible_truncation)]
        let mut count = 0;
        unsafe {
            check(ffi::chfl_selection_evaluate(
                self.as_mut_ptr(),
                frame.as_ptr(),
                &mut count,
            ))
            .expect("failed to evaluate selection");
        }

        let size = count as usize;
        let mut chfl_matches = vec![ffi::chfl_match { size: 0, atoms: [0; 4] }; size];
        unsafe {
            check(ffi::chfl_selection_matches(
                self.handle,
                chfl_matches.as_mut_ptr(),
                count,
            ))
            .expect("failed to extract matches");
        }

        return chfl_matches
            .into_iter()
            .map(|chfl_match| Match {
                size: chfl_match.size as usize,
                atoms: [
                    chfl_match.atoms[0] as usize,
                    chfl_match.atoms[1] as usize,
                    chfl_match.atoms[2] as usize,
                    chfl_match.atoms[3] as usize,
                ],
            })
            .collect();
    }

    /// Evaluates a selection of size 1 on a given `frame`. This function
    /// returns the list of atomic indexes in the frame matching this selection.
    ///
    /// # Panics
    ///
    /// If the selection size is not 1
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Selection, Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("H"), [1.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("O"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("H"), [-1.0, 0.0, 0.0], None);
    ///
    /// let mut selection = Selection::new("name H").unwrap();
    /// let matches = selection.list(&frame);
    ///
    /// assert_eq!(matches.len(), 2);
    /// assert_eq!(matches[0], 0);
    /// assert_eq!(matches[1], 2);
    /// ```
    pub fn list(&mut self, frame: &Frame) -> Vec<usize> {
        assert!(
            self.size() == 1,
            "can not call `Selection::list` on a multiple selection"
        );
        return self.evaluate(frame).into_iter().map(|m| m[0]).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Atom, Topology};

    #[test]
    fn clone() {
        let selection = Selection::new("name H").unwrap();

        let copy = selection.clone();
        assert_eq!(selection.size(), 1);
        assert_eq!(copy.size(), 1);
    }

    fn testing_frame() -> Frame {
        let mut topology = Topology::new();

        topology.add_atom(&Atom::new("H"));
        topology.add_atom(&Atom::new("O"));
        topology.add_atom(&Atom::new("O"));
        topology.add_atom(&Atom::new("H"));

        topology.add_bond(0, 1);
        topology.add_bond(1, 2);
        topology.add_bond(2, 3);

        let mut frame = Frame::new();
        frame.resize(4);
        frame.set_topology(&topology).unwrap();
        return frame;
    }

    mod matches {
        use super::*;

        #[test]
        fn index() {
            let m = Match::new(&[1, 2, 3, 4]);
            assert_eq!(m[0], 1);
            assert_eq!(m[1], 2);
            assert_eq!(m[2], 3);
            assert_eq!(m[3], 4);

            let m = Match::new(&[1, 2]);
            assert_eq!(m[0], 1);
            assert_eq!(m[1], 2);
        }

        #[test]
        fn iter() {
            let match_ = Match::new(&[1, 2, 3, 4]);
            assert_eq!(match_.iter().copied().collect::<Vec<usize>>(), vec![1, 2, 3, 4]);

            let v = [1, 2, 3, 4];
            for (i, &m) in match_.iter().enumerate() {
                assert_eq!(v[i], m);
            }
        }

        #[test]
        #[should_panic(expected = "assertion failed: i < self.len()")]
        fn out_of_bound() {
            let m = Match::new(&[1, 2]);
            let _ = m[2];
        }

        #[test]
        #[should_panic(expected = "assertion failed: atoms.len() <= 4")]
        fn too_big() {
            let _ = Match::new(&[1, 2, 3, 5, 4]);
        }
    }

    #[test]
    fn size() {
        let selection = Selection::new("name H").unwrap();
        assert_eq!(selection.size(), 1);

        let selection = Selection::new("angles: name(#1) H").unwrap();
        assert_eq!(selection.size(), 3);

        let selection = Selection::new("four: name(#1) H").unwrap();
        assert_eq!(selection.size(), 4);
    }

    #[test]
    fn string() {
        let selection = Selection::new("name H").unwrap();
        assert_eq!(selection.string(), "name H");

        let selection = Selection::new("angles: name(#1) H").unwrap();
        assert_eq!(selection.string(), "angles: name(#1) H");
    }

    #[test]
    fn invalid() {
        let error = Selection::new("foo").unwrap_err();
        assert_eq!(error.message, "unexpected identifier 'foo' in mathematical expression");
        assert_eq!(error.status, Status::SelectionError);
    }

    #[test]
    fn evaluate() {
        let frame = testing_frame();

        let mut selection = Selection::new("name H").unwrap();
        let res = selection.evaluate(&frame);
        assert_eq!(res, &[Match::new(&[0]), Match::new(&[3])]);

        let mut selection = Selection::new("angles: all").unwrap();
        let res = selection.evaluate(&frame);
        for m in &[Match::new(&[0, 1, 2]), Match::new(&[1, 2, 3])] {
            assert!(res.iter().any(|r| r == m));
        }
    }

    #[test]
    fn list() {
        let frame = testing_frame();

        let mut selection = Selection::new("name H").unwrap();
        let res = selection.list(&frame);
        assert_eq!(res, vec![0, 3]);
    }

    #[test]
    #[should_panic = "can not call `Selection::list` on a multiple selection"]
    fn list_on_size_1_selection() {
        let frame = testing_frame();
        let mut selection = Selection::new("pairs: name(#1) H").unwrap();
        let _list = selection.list(&frame);
    }
}

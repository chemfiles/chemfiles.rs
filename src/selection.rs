// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C)-2017 2015 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::ops::{Drop, Index};
use std::u64;
use std::iter::IntoIterator;
use std::slice::Iter;

use chemfiles_sys::*;
use errors::{check, Error};
use strings;
use frame::Frame;
use Result;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A `Match` is a set of atomic indexes matching a given selection. It can
/// mostly be used like a `&[u64]`.
pub struct Match(chfl_match);

#[allow(len_without_is_empty)]
impl Match {
    fn zero() -> Match {
        Match(chfl_match {
            size: 0,
            atoms: [0; 4],
        })
    }

    /// Get the length of the Match.
    ///
    /// # Example
    ///
    /// ```
    /// # use chemfiles::Match;
    /// let atomic_match = Match::new(&[3, 4, 5]);
    /// assert_eq!(atomic_match.len(), 3);
    /// ```
    #[allow(cast_possible_truncation)]
    pub fn len(&self) -> usize {
        self.0.size as usize
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
    pub fn new(atoms: &[u64]) -> Match {
        assert!(atoms.len() <= 4);
        let size = atoms.len();
        let mut matches = [u64::max_value(); 4];
        for (i, atom) in atoms.iter().enumerate() {
            matches[i] = *atom;
        }
        Match(chfl_match {
            size: size as u64,
            atoms: matches,
        })
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
    pub fn iter(&self) -> Iter<u64> {
        self.0.atoms[..self.len()].iter()
    }
}

impl Index<usize> for Match {
    type Output = u64;
    fn index(&self, i: usize) -> &u64 {
        assert!(i < self.len());
        &self.0.atoms[i]
    }
}

impl<'a> IntoIterator for &'a Match {
    type Item = &'a u64;
    type IntoIter = Iter<'a, u64>;
    fn into_iter(self) -> Iter<'a, u64> {
        self.0.atoms[..self.len()].into_iter()
    }
}

/// A `Selection` allow to select atoms in a `Frame`, from a selection
/// language. The selection language is built by combining basic operations.
/// Each basic operation follows the `<selector>[(<variable>)] <operator>
/// <value>` structure, where `<operator>` is a comparison operator in
/// `== != < <= > >=`.
pub struct Selection {
    handle: *mut CHFL_SELECTION,
}

impl Clone for Selection {
    fn clone(&self) -> Selection {
        unsafe {
            let new_handle = chfl_selection_copy(self.as_ptr());
            Selection::from_ptr(new_handle).expect("Out of memory when copying a Selection")
        }
    }
}

impl Drop for Selection {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_selection_free(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
        }
    }
}

impl Selection {
    /// Create a `Selection` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *mut CHFL_SELECTION) -> Result<Selection> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Selection { handle: ptr })
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_ptr(&self) -> *const CHFL_SELECTION {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_SELECTION {
        self.handle
    }

    /// Create a new selection from the given selection string.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Selection;
    /// let selection = Selection::new("pairs: name(#1) H and name(#2) O").unwrap();
    /// ```
    pub fn new<'a, S: Into<&'a str>>(selection: S) -> Result<Selection> {
        let buffer = strings::to_c(selection.into());
        unsafe {
            let handle = chfl_selection(buffer.as_ptr());
            Selection::from_ptr(handle)
        }
    }

    /// Get the size of the selection, i.e. the number of atoms we are selecting
    /// together.
    ///
    /// This value is 1 for the 'atom' context, 2 for the 'pair' and 'bond'
    /// context, 3 for the 'three' and 'angles' contextes and 4 for the 'four'
    /// and 'dihedral' contextes.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Selection;
    /// let selection = Selection::new("pairs: name(#1) H and name(#2) O").unwrap();
    /// assert_eq!(selection.size(), Ok(2));
    /// ```
    pub fn size(&self) -> Result<u64> {
        let mut size = 0;
        unsafe {
            try!(check(chfl_selection_size(self.as_ptr(), &mut size)));
        }
        return Ok(size);
    }

    /// Get the selection string used to create this selection.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Selection;
    /// let selection = Selection::new("name H").unwrap();
    /// assert_eq!(selection.string(), Ok(String::from("name H")));
    /// ```
    pub fn string(&self) -> Result<String> {
        let get_string = |ptr, len| unsafe { chfl_selection_string(self.as_ptr(), ptr, len) };
        let selection = try!(strings::call_autogrow_buffer(1024, get_string));
        return Ok(strings::from_c(selection.as_ptr()));
    }

    /// Evaluate a selection for a given frame, and return the corresponding
    /// matches.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Selection, Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("H").unwrap(), [1.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("O").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("H").unwrap(), [-1.0, 0.0, 0.0], None).unwrap();
    ///
    /// let mut selection = Selection::new("pairs: name(#1) H and name(#2) O").unwrap();
    /// let matches = selection.evaluate(&frame).unwrap();
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
    pub fn evaluate(&mut self, frame: &Frame) -> Result<Vec<Match>> {
        let mut matches_count = 0;
        unsafe {
            try!(check(
                chfl_selection_evaluate(self.as_mut_ptr(), frame.as_ptr(), &mut matches_count)
            ));
        }

        #[allow(cast_possible_truncation)]
        let mut matches = vec![Match::zero(); matches_count as usize];
        unsafe {
            try!(check(chfl_selection_matches(
                self.handle,
                matches.as_mut_ptr() as *mut _,
                matches_count
            )));
        }
        return Ok(matches);
    }

    /// Evaluates a selection of size 1 on a given `frame`. This function
    /// returns the list of atomic indexes in the frame matching this selection.
    ///
    /// # Panics
    ///
    /// If the selection size is not 1.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Selection, Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("H").unwrap(), [1.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("O").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("H").unwrap(), [-1.0, 0.0, 0.0], None).unwrap();
    ///
    /// let mut selection = Selection::new("name H").unwrap();
    /// let matches = selection.list(&frame).unwrap();
    ///
    /// assert_eq!(matches.len(), 2);
    /// assert_eq!(matches[0], 0);
    /// assert_eq!(matches[1], 2);
    /// ```
    pub fn list(&mut self, frame: &Frame) -> Result<Vec<u64>> {
        let matches = try!(self.evaluate(frame));
        let mut list = vec![0; matches.len()];
        for (i, m) in matches.iter().enumerate() {
            list[i] = m[0];
        }
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Frame;
    use Topology;
    use Atom;

    #[test]
    fn clone() {
        let selection = Selection::new("name H").unwrap();

        let copy = selection.clone();
        assert_eq!(selection.size(), Ok(1));
        assert_eq!(copy.size(), Ok(1));
    }

    fn testing_frame() -> Frame {
        let mut topology = Topology::new().unwrap();

        topology.add_atom(&Atom::new("H").unwrap()).unwrap();
        topology.add_atom(&Atom::new("O").unwrap()).unwrap();
        topology.add_atom(&Atom::new("O").unwrap()).unwrap();
        topology.add_atom(&Atom::new("H").unwrap()).unwrap();

        topology.add_bond(0, 1).unwrap();
        topology.add_bond(1, 2).unwrap();
        topology.add_bond(2, 3).unwrap();

        let mut frame = Frame::new().unwrap();
        frame.resize(4).unwrap();
        frame.set_topology(&topology).unwrap();
        return frame;
    }

    mod matches {
        use super::*;

        #[test]
        fn size_of() {
            assert_eq!(::std::mem::size_of::<chfl_match>(), ::std::mem::size_of::<Match>())
        }

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
            assert_eq!(match_.iter().cloned().collect::<Vec<u64>>(), vec![1, 2, 3, 4]);

            let v = vec![1, 2, 3, 4];
            let mut i = 0;
            for &m in &match_ {
                assert_eq!(v[i], m);
                i += 1;
            }
        }

        #[test]
        #[should_panic]
        fn out_of_bound() {
            let m = Match::new(&[1, 2]);
            let _ = m[2];
        }

        #[test]
        #[should_panic]
        fn too_big() {
            let _ = Match::new(&[1, 2, 3, 5, 4]);
        }
    }

    #[test]
    fn size() {
        let selection = Selection::new("name H").unwrap();
        assert_eq!(selection.size(), Ok(1));

        let selection = Selection::new("angles: name(#1) H").unwrap();
        assert_eq!(selection.size(), Ok(3));

        let selection = Selection::new("four: name(#1) H").unwrap();
        assert_eq!(selection.size(), Ok(4));
    }

    #[test]
    fn string() {
        let selection = Selection::new("name H").unwrap();
        assert_eq!(selection.string().unwrap(), "name H");

        let selection = Selection::new("angles: name(#1) H").unwrap();
        assert_eq!(selection.string().unwrap(), "angles: name(#1) H");
    }

    #[test]
    fn evaluate() {
        let frame = testing_frame();

        let mut selection = Selection::new("name H").unwrap();
        let res = selection.evaluate(&frame).unwrap();
        assert_eq!(res, &[Match::new(&[0]), Match::new(&[3])]);

        let mut selection = Selection::new("angles: all").unwrap();
        let res = selection.evaluate(&frame).unwrap();
        for m in &[Match::new(&[0, 1, 2]), Match::new(&[1, 2, 3])] {
            assert!(res.iter().find(|&r| r == m).is_some())
        }
    }

    #[test]
    fn list() {
        let frame = testing_frame();

        let mut selection = Selection::new("name H").unwrap();
        let res = selection.list(&frame).unwrap();
        assert_eq!(res, vec![0, 3]);
    }
}

/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::ops::{Drop, Index};
use std::usize;
use std::iter::IntoIterator;
use std::slice::Iter;

use chemfiles_sys::*;
use errors::{check, Error};
use string;
use frame::Frame;
use Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A `Match` is a set of atomic indexes matching a given selection. It should
/// be used like a `&[usize]`.
pub struct Match(chfl_match_t);

impl Match {
    fn zero() -> Match {
        Match(chfl_match_t{size: 0, atoms: [0; 4]})
    }

    fn len(&self) -> usize {
        self.0.size as usize
    }

    /// Create a new match containing the atoms in the `atoms` slice.
    ///
    /// # Panics
    ///
    /// If the slice contains more than 4 elements, which is the maximal size
    /// of a match.
    pub fn new(atoms: &[usize]) -> Match {
        assert!(atoms.len() <= 4);
        let size = atoms.len() as i8;
        let mut matches = [usize::MAX; 4];
        for (i, atom) in atoms.iter().enumerate() {
            matches[i] = *atom;
        }
        Match(chfl_match_t{size: size, atoms: matches})
    }

    /// Iterate over the atomic indexes in the match.
    pub fn iter<'a>(&'a self) -> Iter<'a, usize> {
        self.0.atoms[..self.len()].iter()
    }
}

impl Index<usize> for Match {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        assert!(i < self.len(), "");
        unsafe {
            self.0.atoms.get_unchecked(i)
        }
    }
}

impl<'a> IntoIterator for &'a Match {
    type Item = &'a usize;
    type IntoIter = Iter<'a, usize>;
    fn into_iter(self) -> Iter<'a, usize> {
        self.0.atoms[..self.len()].into_iter()
    }
}

/******************************************************************************/

/// Select atoms in a `Frame` with a selection language.
///
/// The selection language is built by combining basic operations. Each basic
/// operation follows the `<selector>[(<variable>)] <operator> <value>`
/// structure, where `<operator>` is a comparison operator in `== != < <= > >=`.
/// Refer to the [full
/// documentation](http://chemfiles.rtfd.io/en/latest/selections.html) to know
/// the allowed selectors and how to use them.
pub struct Selection {
    handle: *const CHFL_SELECTION
}

impl Drop for Selection {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_selection_free(self.handle as *mut CHFL_SELECTION)
            ).ok().expect("Error while freeing memory!");
        }
    }
}

impl Selection {
    /// Create a new selection from the given selection string.
    pub fn new<'a, S: Into<&'a str>>(selection: S) -> Result<Selection> {
        let handle : *const CHFL_SELECTION;
        let buffer = string::to_c(selection.into());
        unsafe {
            handle = chfl_selection(buffer.as_ptr());
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Selection{handle: handle})
        }
    }

    /// Get the size of the selection, i.e. the number of atoms we are selecting
    /// together.
    ///
    /// This value is 1 for the 'atom' context, 2 for the 'pair' and 'bond'
    /// context, 3 for the 'three' and 'angles' contextes and 4 for the 'four'
    /// and 'dihedral' contextes.
    pub fn size(&self) -> Result<usize> {
        let mut size = 0;
        unsafe {
            try!(check(chfl_selection_size(self.handle, &mut size)));
        }
        return Ok(size);
    }

    /// Evaluate a selection for a given frame, and return the corresponding
    /// matches.
    pub fn evaluate(&mut self, frame: &Frame) -> Result<Vec<Match>> {
        let mut n_matches = 0;
        unsafe {
            try!(check(chfl_selection_evalutate(
                self.handle as *mut CHFL_SELECTION,
                frame.as_ptr(),
                &mut n_matches
            )));
        }

        let mut matches = vec![Match::zero(); n_matches];
        unsafe {
            try!(check(chfl_selection_matches(
                self.handle,
                matches.as_mut_ptr() as *mut chfl_match_t,
                n_matches
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
    pub fn list(&mut self, frame: &Frame) -> Result<Vec<usize>> {
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
    pub use super::*;
    pub use chemfiles_sys::chfl_match_t;
    use Frame;
    use Topology;
    use Atom;

    fn testing_frame() -> Frame {
        let mut topology = Topology::new().unwrap();

        topology.push(&Atom::new("H").unwrap()).unwrap();
        topology.push(&Atom::new("O").unwrap()).unwrap();
        topology.push(&Atom::new("O").unwrap()).unwrap();
        topology.push(&Atom::new("H").unwrap()).unwrap();

        topology.add_bond(0, 1).unwrap();
        topology.add_bond(1, 2).unwrap();
        topology.add_bond(2, 3).unwrap();

        let mut frame = Frame::new(4).unwrap();
        frame.set_topology(&topology).unwrap();
        return frame;
    }

    mod matches {
        use super::*;

        #[test]
        fn size_of() {
            assert_eq!(
                ::std::mem::size_of::<chfl_match_t>(),
                ::std::mem::size_of::<Match>()
            )
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
        #[should_panic]
        fn out_of_bound() {
            let m = Match::new(&[1, 2]);
            m[2];
        }

        #[test]
        #[should_panic]
        fn too_big() {
            Match::new(&[1, 2, 3, 5, 4]);
        }
    }

    #[test]
    fn size() {
        let sel = Selection::new("name H").unwrap();
        assert_eq!(sel.size(), Ok(1));

        let sel = Selection::new("angles: name($1) H").unwrap();
        assert_eq!(sel.size(), Ok(3));

        let sel = Selection::new("four: name($1) H").unwrap();
        assert_eq!(sel.size(), Ok(4));
    }

    #[test]
    fn evaluate() {
        let frame = testing_frame();

        let mut sel = Selection::new("name H").unwrap();
        let res = sel.evaluate(&frame).unwrap();
        assert_eq!(res, &[Match::new(&[0]), Match::new(&[3])]);

        let mut sel = Selection::new("angles: all").unwrap();
        let res = sel.evaluate(&frame).unwrap();
        for m in &[Match::new(&[0, 1, 2]), Match::new(&[1, 2, 3])] {
            assert!(res.iter().find(|&r| r == m).is_some())
        }
    }

    #[test]
    fn list() {
        let frame = testing_frame();

        let mut sel = Selection::new("name H").unwrap();
        let res = sel.list(&frame).unwrap();
        assert_eq!(res, vec![0, 3]);
    }
}

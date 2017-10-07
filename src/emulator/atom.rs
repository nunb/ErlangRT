//! Global Atom storage.
//! Global is ugly, but necessary for now. If we ever create more than 1 VM,
//! this will have to be shared somehow.
use std::collections::BTreeMap;
use std::sync::Mutex;

use defs::Word;
use term::lterm::LTerm;

/// Lookup table for atom to atom index and back. Declared static for use by
/// printing and atom loading facilities without having to pass the VM pointer
/// all the way down.
struct AtomStorage {
  /// Direct mapping string to atom index
  atoms: Mutex<BTreeMap<String, Word>>,

  /// Reverse mapping atom index to string (sorted by index)
  atoms_r: Mutex<Vec<String>>,
}

lazy_static! {
  static ref ATOMS: AtomStorage = {
    AtomStorage {
      atoms: Mutex::new(BTreeMap::new()),
      atoms_r: Mutex::new(Vec::new()),
    }
  };
}


// Allocate new atom in the atom table or find existing. Pack the atom index
// as an immediate2 Term
pub fn from_str(val: &str) -> LTerm {
  let mut atoms_ = ATOMS.atoms.lock().unwrap();

  if atoms_.contains_key(val) {
    //println!("atom {} found {}", val, self.atoms[val]);
    return LTerm::make_atom(atoms_[val]);
  }

  let mut atoms_r_ = ATOMS.atoms_r.lock().unwrap();
  let index = atoms_r_.len();

  let val1 = String::from(val);
  atoms_.entry(val1).or_insert(index);

  let val2 = String::from(val);
  atoms_r_.push(val2);

  //println!("atom {} new {}", val, index);
  LTerm::make_atom(index)
}


pub fn to_str(a: LTerm) -> String {
  assert!(a.is_atom());
  let atoms_r_ = ATOMS.atoms_r.lock().unwrap();
  atoms_r_[a.atom_index()].to_string()
}

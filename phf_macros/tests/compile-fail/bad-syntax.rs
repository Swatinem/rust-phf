#![feature(proc_macro_hygiene)]

extern crate phf;

use phf::phf_map;

static MAP: phf::Map<u32, u32> = phf_map! {
    Signature::
    => //~ ERROR expected identifier
    ()
};

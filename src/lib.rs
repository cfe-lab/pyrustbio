#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
extern crate bio;

use pyo3::prelude::*;

use bio::alignment::distance::levenshtein;
use bio::alignment::pairwise::Aligner;
use bio::alignment::{Alignment, AlignmentOperation};
use bio::scores::blosum62::blosum62;

#[py::modinit(_pyrustbio)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "levenshtein")]
    fn py_levenshtein(_py: Python, a: String, b: String) -> PyResult<(u32)> {
        Ok(levenshtein(&(a.into_bytes()), &(b.into_bytes())))
    }

    #[pyfn(m, "affine")]
    fn py_affine(_py: Python, r: String, q: String,
                  gop: i32, gep: i32) -> PyResult<&PyList> {
        let alnmnt = affine(r.into_bytes(), q.into_bytes(), gop, gep);
        let ops: Vec<i32> = alnmnt.operations.into_iter().map(op2int).collect();

        Ok(PyList::new(_py, &ops))
    }

    Ok(())
}

fn op2int(op: AlignmentOperation) -> i32 {
    match op {
        AlignmentOperation::Match => 0,
        AlignmentOperation::Subst => 1,
        AlignmentOperation::Del => 2,
        AlignmentOperation::Ins => 3,
        AlignmentOperation::Xclip(_) => 4,
        AlignmentOperation::Yclip(_) => 5
    }
}

fn affine(r: Vec<u8>, q: Vec<u8>, gop: i32, gep: i32) -> Alignment {
    let mut a = Aligner::with_capacity(r.len(), q.len(), gop, gep, &blosum62);
    let alnmnt = a.global(&r, &q);
    alnmnt
}

#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
extern crate bio;

use pyo3::prelude::*;

use bio::alignment::distance::levenshtein;
use bio::alignment::pairwise::Aligner;
use bio::scores::blosum62::blosum62;

#[py::modinit(_pyrustbio)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "levenshtein")]
    fn py_levenshtein(_py: Python, a: String, b: String) -> PyResult<(u32)> {
        Ok(levenshtein(&(a.into_bytes()), &(b.into_bytes())))
    }

    #[pyfn(m, "affine")]
    fn py_affine(_py: Python, q: String, r: String,
                  gop: i32, gep: i32) -> PyResult<&PyList> {
        let mut a = Aligner::with_capacity(q.len(), r.len(), gop, gep, &blosum62);
        let (q_b, r_b) = (q.into_bytes(), r.into_bytes());
        let alnmnt = a.semiglobal(&q_b, &r_b);

        let t = [alnmnt.score.to_object(_py),
                 alnmnt.pretty(&q_b, &r_b).to_object(_py)];
        Ok(PyList::new(_py, &t))
    }

    Ok(())
}

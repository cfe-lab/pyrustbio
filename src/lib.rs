#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
extern crate bio;

use pyo3::prelude::*;

use bio::alignment::distance::levenshtein;
use bio::alignment::pairwise::Aligner;
use bio::alignment::AlignmentOperation;
use bio::alignment::AlignmentMode;
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

        let mut x_pretty = String::new();
        let mut y_pretty = String::new();

        if !alnmnt.operations.is_empty() {
            let mut x_i: usize;
            let mut y_i: usize;

            // If the alignment mode is one of the standard ones, the prefix clipping is
            // implicit so we need to process it here
            match alnmnt.mode {
                AlignmentMode::Custom => {
                    x_i = 0;
                    y_i = 0;
                }
                _ => {
                    x_i = alnmnt.xstart;
                    y_i = alnmnt.ystart;
                    for k in 0..x_i {
                        x_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[q_b[k]])));
                        y_pretty.push('-')
                    }
                    for k in 0..y_i {
                        y_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[r_b[k]])));
                        x_pretty.push('-')
                    }
                }
            }

            // Process the alignment.
            for i in 0..alnmnt.operations.len() {
                match alnmnt.operations[i] {
                    AlignmentOperation::Match => {
                        x_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[q_b[x_i]])));
                        x_i += 1;

                        y_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[r_b[y_i]])));
                        y_i += 1;
                    }
                    AlignmentOperation::Subst => {
                        x_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[q_b[x_i]])));
                        x_i += 1;

                        y_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[r_b[y_i]])));
                        y_i += 1;
                    }
                    AlignmentOperation::Del => {
                        x_pretty.push('-');

                        y_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[r_b[y_i]])));
                        y_i += 1;
                    }
                    AlignmentOperation::Ins => {
                        x_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[q_b[x_i]])));
                        x_i += 1;

                        y_pretty.push('-');
                    }
                    AlignmentOperation::Xclip(len) => {
                        for k in 0..len {
                            x_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[q_b[k]])));
                            x_i += 1;

                            y_pretty.push('-')
                        }
                    }
                    AlignmentOperation::Yclip(len) => {
                        for k in 0..len {
                            y_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[r_b[k]])));
                            y_i += 1;

                            x_pretty.push('-')
                        }
                    }
                }
            }

            // If the alignment mode is one of the standard ones, the suffix clipping is
            // implicit so we need to process it here
            match alnmnt.mode {
                AlignmentMode::Custom => {}
                _ => {
                    for k in x_i..alnmnt.xlen {
                        x_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[q_b[k]])));
                        y_pretty.push('-')
                    }
                    for k in y_i..alnmnt.ylen {
                        y_pretty.push_str(&format!("{}", String::from_utf8_lossy(&[r_b[k]])));
                        x_pretty.push('-')
                    }
                }
            }
        }
        let t = [alnmnt.score.to_object(_py),
                 x_pretty.to_object(_py),
                 y_pretty.to_object(_py)];
        Ok(PyList::new(_py, &t))
    }

    Ok(())
}

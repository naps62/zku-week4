use ark_bls12_381::Fq2 as F;
use nalgebra::base::{DMatrix, DVector};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

type FMatrix = DMatrix<F>;

struct Freivald {
    rng: ThreadRng,
    array_size: usize,
    n_runs: usize,
}

impl Freivald {
    fn new(array_size: usize) -> Self {
        Self {
            array_size,
            rng: thread_rng(),
            n_runs: usize::pow(2, array_size as u32),
        }
    }

    fn verify(&mut self, matrix_a: &FMatrix, matrix_b: &FMatrix, supposed_ab: &FMatrix) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));

        for _ in 0..self.n_runs {
            let v = self.gen_v();

            if !(matrix_a * matrix_b * &v == supposed_ab * &v) {
                return false;
            };
        }

        println!("here");
        true
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    fn verify_once(matrix_a: &FMatrix, matrix_b: &FMatrix, supposed_ab: &FMatrix) -> bool {
        let mut freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }

    fn gen_v(&mut self) -> DVector<F> {
        let r = self.rng.gen::<u32>();
        let iter = (0..self.array_size).map(|i| r.pow(i as u32).into());
        DVector::from_iterator(self.array_size, iter)
    }
}

// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    todo!()
}

pub fn check_matrix_dimensions(a: &FMatrix, b: &FMatrix, ab: &FMatrix) -> bool {
    a.nrows() == b.nrows()
        && a.ncols() == b.ncols()
        && a.nrows() == ab.nrows()
        && a.ncols() == ab.ncols()
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        static ref MATRIX_A: FMatrix =
            DMatrix::from_row_slice(2, 2, &[2.into(), 3.into(), 3.into(), 4.into()]);
        static ref MATRIX_A_DOT_A: FMatrix =
            DMatrix::from_row_slice(2, 2, &[13.into(), 18.into(), 18.into(), 25.into()]);
        static ref MATRIX_B: FMatrix =
            DMatrix::from_row_slice(2, 2, &[1.into(), 0.into(), 1.into(), 2.into()]);
        static ref MATRIX_B_DOT_B: FMatrix =
            DMatrix::from_row_slice(2, 2, &[1.into(), 0.into(), 3.into(), 4.into()]);
        static ref MATRIX_C: FMatrix =
            DMatrix::from_row_slice(2, 2, &[6.into(), 5.into(), 8.into(), 7.into()]);
        static ref MATRIX_C_DOT_C: FMatrix =
            DMatrix::from_row_slice(2, 2, &[76.into(), 65.into(), 104.into(), 89.into()]);
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] a: &FMatrix,
        #[case] b: &FMatrix,
        #[case] ab: &FMatrix,
    ) {
        let mut freivald = Freivald::new(ab.nrows());
        assert!(freivald.verify(a, b, ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(#[case] a: &FMatrix, #[case] b: &FMatrix, #[case] c: &FMatrix) {
        assert!(!Freivald::verify_once(a, b, c))
    }
}

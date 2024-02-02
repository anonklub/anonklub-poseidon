pub mod constants;
pub mod sponge;

use ark_ff::Field;

#[derive(Clone)]
pub struct PoseidonConstants<F: Field> {
    pub round_keys: Vec<F>,
    pub mds_matrix: Vec<Vec<F>>,
    pub num_full_rounds: usize,
    pub num_partial_rounds: usize,
}

const CAPACITY: usize = 1; // We fix the capacity to be one.

#[derive(Clone)]
pub struct Poseidon<F: Field, const WIDTH: usize> {
    pub state: [F; WIDTH],
    pub constants: PoseidonConstants<F>,
    pub pos: usize,
}

impl<F: Field, const WIDTH: usize> Poseidon<F, WIDTH> {
    pub fn new(constants: PoseidonConstants<F>) -> Self {
        let state = [F::zero(); WIDTH];
        Self {
            state,
            constants,
            pos: 0,
        }
    }

    pub fn permute(&mut self) {
        let full_rounds_half = self.constants.num_full_rounds / 2;

        // First half of full rounds
        for _ in 0..full_rounds_half {
            self.full_round();
        }

        // Partial rounds
        for _ in 0..self.constants.num_partial_rounds {
            self.partial_round();
        }

        // Second half of full rounds
        for _ in 0..full_rounds_half {
            self.full_round();
        }
    }

    pub fn reset(&mut self) {
        self.state = [F::zero(); WIDTH];
        self.pos = 0;
    }

    fn add_constants(&mut self) {
        // Add round constants
        for i in 0..self.state.len() {
            self.state[i] += self.constants.round_keys[i + self.pos];
        }
    }

    // MDS matrix multiplication
    fn matrix_mul(&mut self) {
        let mut result = [F::zero(); WIDTH];
        for (i, val) in self.constants.mds_matrix.iter().enumerate() {
            let mut tmp = F::zero();
            for (j, element) in self.state.iter().enumerate() {
                tmp += val[j] * element
            }
            result[i] = tmp;
        }

        self.state = result;
    }

    fn full_round(&mut self) {
        let t = self.state.len();
        self.add_constants();

        // S-boxes
        for i in 0..t {
            self.state[i] = self.state[i].square().square() * self.state[i];
        }

        self.matrix_mul();

        // Update the position of the round constants that are added
        self.pos += self.state.len();
    }

    fn partial_round(&mut self) {
        self.add_constants();

        // S-box
        self.state[0] = self.state[0].square().square() * self.state[0];

        self.matrix_mul();

        // Update the position of the round constants that are added
        self.pos += self.state.len();
    }
}

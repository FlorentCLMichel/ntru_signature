use ntru_signature::*;

fn main() {

    // length of each vector
    let len_arrays: usize = 40;

    // number of vectors
    let n_arrays: usize = 40;

    // create the vectors
    let mut x: Vec<Vec<u8>> = vec![vec![0; len_arrays]; n_arrays];

    // fill them with random values
    for v in &mut x { fill_rand_bytes(v); }

    // print the results
    for v in x {
        for e in v {
            print!("{:02X} ", e);
        }
        println!("");
    }
}

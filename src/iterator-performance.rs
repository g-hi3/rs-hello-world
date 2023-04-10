fn main() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    // The Rust compiler "unrolls" this loop, because the number of iterations is known at compile time.
    // This means, after compilation, there will be no loop left and the code is executed 12 times.
    // Rust also places the coefficients in registers.
    // This means that the access is very fast and also that there is no bounds check necessary.
    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
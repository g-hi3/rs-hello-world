fn main() {
    // The example leaves these values empty, but it doesn't compile that way, so I added some.
    let buffer: &mut [i32] = &mut [32, 16, 8, 4, 2, 1];
    let coefficients: [i64; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let qlp_shift: i16 = 96;

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
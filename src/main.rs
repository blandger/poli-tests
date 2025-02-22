use poli_tests::Polynomial;

fn main() {
    // Define the polynomials from the example
    let g = Polynomial::new(vec![1, 2, 3, 4]);  // 1 + 2x + 3x^2 + 4x^3
    let h = Polynomial::new(vec![5, 6, 7, 8]);  // 5 + 6x + 7x^2 + 8x^3

    println!("G(x) = {}", g);
    println!("H(x) = {}", h);

    // Calculate regular polynomial multiplication
    let y = g.multiply(&h);
    println!("\nRegular multiplication Y(x) = {}", y);

    // Calculate cyclic convolution
    let pwc = g.cyclic_convolution(&h);
    println!("\nCyclic convolution PWC(x) = {}", pwc);

    // Calculate negacyclic convolution
    let nwc = g.negacyclic_convolution(&h);
    println!("\nNegacyclic convolution NWC(x) = {}", nwc);

    // Print coefficients in vector notation
    println!("\nCoefficients in vector notation:");
    println!("g = {:?}", g.coefficients);
    println!("h = {:?}", h.coefficients);
    println!("Y(x) = {:?}", y.coefficients);
    println!("PWC(x) = {:?}", pwc.coefficients);
    println!("NWC(x) = {:?}", nwc.coefficients);
}

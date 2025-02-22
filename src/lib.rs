use std::fmt;

/// Represents a polynomial with integer coefficients
#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<i32>,
}

impl Polynomial {
    /// Creates a new polynomial from a vector of coefficients
    pub fn new(coefficients: Vec<i32>) -> Self {
        Self { coefficients }
    }

    /// Multiplies two polynomials
    pub fn multiply(&self, other: &Polynomial) -> Self {
        let n1 = self.coefficients.len();
        let n2 = other.coefficients.len();
        let mut result = vec![0; n1 + n2 - 1];

        for (i, &coef1) in self.coefficients.iter().enumerate() {
            for (j, &coef2) in other.coefficients.iter().enumerate() {
                result[i + j] += coef1 * coef2;
            }
        }

        Self::new(result)
    }

    /// Computes cyclic convolution (PWC) with modulo x^n - 1
    pub fn cyclic_convolution(&self, other: &Polynomial) -> Self {
        println!("Start Cyclic convolution PWC...");
        // First, compute regular polynomial multiplication
        let product = self.multiply(other);

        // The size of the result should be max(self.len, other.len)
        let n = self.coefficients.len().max(other.coefficients.len());
        let mut pwc = vec![0; n];
        println!("PWC ( n = {n})");

        // Apply modulo x^n - 1 by wrapping coefficients
        for (i, &coef) in product.coefficients.iter().enumerate() {
            let wrapped_index = i % n;
            let new_pwc_value = pwc[wrapped_index] + coef;
            println!("i: {}, coef: {}, wrapped_index: [{}] = {new_pwc_value} ({} + {})", i, coef, &wrapped_index, pwc[wrapped_index], coef);
            // pwc[wrapped_index] += coef;
            pwc[wrapped_index] = new_pwc_value;
        }

        Self::new(pwc)
    }

    /// Computes negacyclic convolution (NWC) with modulo x^n + 1
    pub fn negacyclic_convolution(&self, other: &Polynomial) -> Self {
        println!("Start Negacyclic convolution NWC...");
        // First, compute regular polynomial multiplication
        let product = self.multiply(other);

        // The size of the result should be max(self.len, other.len)
        let n = self.coefficients.len().max(other.coefficients.len());
        let mut nwc = vec![0; n];
        println!("NWC ( n = {n})");

        // Apply modulo x^n + 1 by wrapping coefficients
        // For x^n + 1, when we wrap around, we need to subtract instead of add
        for (i, &coef) in product.coefficients.iter().enumerate() {
            let wrapped_index = i % n;
            let new_nwc_value = if i < n {
                // Terms before x^n are added normally
                // nwc[wrapped_index] += coef;
                nwc[wrapped_index] + coef
            } else {
                // Terms x^n and higher are subtracted when wrapped
                // Due to modulo x^n + 1, each wrap changes the sign
                if (i / n) % 2 == 1 {
                    // nwc[wrapped_index] -= coef;
                    nwc[wrapped_index] - coef
                } else {
                    // nwc[wrapped_index] += coef;
                    nwc[wrapped_index] + coef
                }
            };
            println!("i: {}, coef: {}, wrapped_index: [{}] = {new_nwc_value} ({} + {})", i, coef, &wrapped_index, nwc[wrapped_index], coef);
            nwc[wrapped_index] = new_nwc_value;
        }

        Self::new(nwc)
    }
}

// Implement Display for pretty printing polynomials
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms: Vec<String> = self.coefficients
            .iter()
            .enumerate()
            .filter(|&(_, coef)| *coef != 0)
            .map(|(power, &coef)| {
                match power {
                    0 => coef.to_string(),
                    1 => format!("{}x", coef),
                    _ => format!("{}x^{}", coef, power),
                }
            })
            .collect();

        if terms.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", terms.join(" + "))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_multiplication() {
        let g = Polynomial::new(vec![1, 2, 3, 4]);
        let h = Polynomial::new(vec![5, 6, 7, 8]);
        let y = g.multiply(&h);
        assert_eq!(y.coefficients, vec![5, 16, 34, 60, 61, 52, 32]);
    }

    #[test]
    fn test_cyclic_convolution() {
        let g = Polynomial::new(vec![1, 2, 3, 4]);
        let h = Polynomial::new(vec![5, 6, 7, 8]);
        let pwc = g.cyclic_convolution(&h);
        assert_eq!(pwc.coefficients, vec![66, 68, 66, 60]);
    }

    #[test]
    fn test_negacyclic_convolution() {
        let g = Polynomial::new(vec![1, 2, 3, 4]);
        let h = Polynomial::new(vec![5, 6, 7, 8]);
        let nwc = g.negacyclic_convolution(&h);

        // For Y(x) = 5 + 16x + 34x² + 60x³ + 61x⁴ + 52x⁵ + 32x⁶
        // mod (x⁴ + 1):
        // x⁴ = -1
        // x⁵ = -x
        // x⁶ = -x²
        // Therefore:
        // x⁰: 5 - 61 = -56
        // x¹: 16 - 52 = -36
        // x²: 34 - 32 = 2
        // x³: 60 = 60
        assert_eq!(nwc.coefficients, vec![-56, -36, 2, 60]);
    }
}

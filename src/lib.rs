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
        // First, compute regular polynomial multiplication
        let product = self.multiply(other);

        // The size of the result should be max(self.len, other.len)
        let n = self.coefficients.len().max(other.coefficients.len());
        let mut pwc = vec![0; n];

        // Apply modulo x^n - 1 by wrapping coefficients
        for (i, &coef) in product.coefficients.iter().enumerate() {
            let wrapped_index = i % n;
            pwc[wrapped_index] += coef;
        }

        Self::new(pwc)
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
}

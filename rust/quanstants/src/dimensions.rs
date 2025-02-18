use std::fmt;
use std::ops;

fn generate_superscript(integer: i8) -> String {
    let int_string = integer.to_string();
    let mut output = String::new();
    for ch in int_string.chars() {
        output.push(char_to_superscript(ch));
    }
    output
}

fn char_to_superscript(character: char) -> char {
    match character {
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '0' => '⁰',
        '-' => '⁻',
        _ => panic!(),
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
#[allow(non_snake_case)]
pub struct Dimensions {
    pub L: i8,
    pub M: i8,
    pub T: i8,
    pub I: i8,
    pub Θ: i8,
    pub N: i8,
    pub J: i8,
}

impl Dimensions {
    #[allow(non_snake_case)]
    pub fn new(L: i8, M: i8, T: i8, I: i8, Θ: i8, N: i8, J: i8) -> Self {
        Self {
            L,
            M,
            T,
            I,
            Θ,
            N,
            J,
        }
    }
}

impl ops::Mul for Dimensions {
    type Output = Self;

    fn mul(self, other: Dimensions) -> Dimensions {
        Self {
            L: self.L + other.L,
            M: self.M + other.M,
            T: self.T + other.T,
            I: self.I + other.I,
            Θ: self.Θ + other.Θ,
            N: self.N + other.N,
            J: self.J + other.J,
        }
    }
}

impl ops::Div for Dimensions {
    type Output = Self;

    fn div(self, other: Dimensions) -> Dimensions {
        Self {
            L: self.L - other.L,
            M: self.M - other.M,
            T: self.T - other.T,
            I: self.I - other.I,
            Θ: self.Θ - other.Θ,
            N: self.N - other.N,
            J: self.J - other.J,
        }
    }
}

impl Dimensions {
    pub fn pow(&self, exp: i8) -> Dimensions {
        Dimensions {
            L: self.L * exp,
            M: self.M * exp,
            T: self.T * exp,
            I: self.I * exp,
            Θ: self.Θ * exp,
            N: self.N * exp,
            J: self.J * exp,
        }
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let exponents = [self.L, self.M, self.T, self.I, self.Θ, self.N, self.J];
        let symbols = ["L", "M", "T", "I", "Θ", "N", "J"];
        let output = if exponents.iter().all(|&x| x == 0) {
            String::from("(dimensionless)")
        } else {
            let mut string = String::new();
            for i in 0..7 {
                if exponents[i] != 0 {
                    string.push_str(symbols[i]);
                    if exponents[i] != 1 {
                        string.push_str(&generate_superscript(exponents[i]));
                    }
                }
            }
            string
        };
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_dimensions() {
        let dim1 = Dimensions::default();
        assert_eq!(dim1, Dimensions::new(0, 0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn mul() {
        let dim1 = Dimensions::new(0, 1, 0, 2, 0, 0, 0);
        let dim2 = Dimensions::new(2, 0, 0, 2, 0, 0, 0);
        assert_eq!(dim1 * dim2, Dimensions::new(2, 1, 0, 4, 0, 0, 0))
    }

    #[test]
    fn div() {
        let dim1 = Dimensions::new(0, 1, 0, 2, 0, 0, 0);
        let dim2 = Dimensions::new(2, 0, 0, 2, 0, 0, 0);
        assert_eq!(dim1 / dim2, Dimensions::new(-2, 1, 0, 0, 0, 0, 0))
    }

    #[test]
    fn pow() {
        let dim1 = Dimensions::new(0, 1, 0, 2, 0, 0, 0);
        assert_eq!(dim1.pow(2), Dimensions::new(0, 2, 0, 4, 0, 0, 0))
    }
}

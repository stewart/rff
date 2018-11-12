use std::ops::{Index, IndexMut};

/// A type shorthand for the lookup key we'll be using, a width/height tuple.
type Idx = (usize, usize);

/// A 2-dimensional matrix of f64 values.
#[derive(Clone, Debug)]
pub struct Matrix {
    width: usize,
    contents: Vec<f64>,
}

impl Matrix {
    /// Creates a new Matrix with the provided width and height.
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            contents: vec![0.0; width * height],
        }
    }
}

impl Index<Idx> for Matrix {
    type Output = f64;

    fn index(&self, (width, height): Idx) -> &Self::Output {
        &self.contents[height * self.width + width]
    }
}

impl IndexMut<Idx> for Matrix {
    fn index_mut(&mut self, (width, height): Idx) -> &mut Self::Output {
        &mut self.contents[height * self.width + width]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut matrix = Matrix::new(1024, 768);

        // Index<Idx>
        assert_eq!(matrix[(1023, 767)], 0.0);

        // IndexMut<Idx>
        matrix[(12, 24)] = 123.456;
        assert_eq!(matrix[(12, 24)], 123.456);
        assert_eq!(matrix[(24, 12)], 0.0);
    }
}

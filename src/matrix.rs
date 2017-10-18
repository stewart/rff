/// The Matrix type represents a 2-dimensional Matrix.
pub struct Matrix {
    cols: usize,
    contents: Vec<f64>
}

impl Matrix {
    /// Creates a new Matrix with the given width and height
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            contents: vec![0.0; width * height],
            cols: width,
        }
    }

    /// Returns a reference to the specified coordinates of the Matrix
    pub fn get(&self, col: usize, row: usize) -> &f64 {
        debug_assert!(col * row < self.contents.len());
        self.contents.get(row * self.cols + col).unwrap()
    }

    /// Sets the coordinates of the Matrix to the specified value
    pub fn set(&mut self, col: usize, row: usize, val: f64) {
        debug_assert!(col * row < self.contents.len());
        self.contents[row * self.cols + col] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mat = Matrix::new(10, 5);
        assert_eq!(mat.get(0, 0), &0.0);
        assert_eq!(mat.get(9, 4), &0.0);
    }

    #[test]
    fn test_set() {
        let mut mat = Matrix::new(10, 5);
        mat.set(9, 4, 1.0);
        assert_eq!(mat.get(0, 0), &0.0);
        assert_eq!(mat.get(9, 4), &1.0);
    }
}

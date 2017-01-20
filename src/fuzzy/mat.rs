/// The Mat struct represents a matrix of scores as a 2D vector of f64s
#[derive(Debug)]
pub struct Mat {
    cols: usize,
    contents: Vec<f64>
}

impl Mat {
    /// Creates a new Mat with the specified dimensions
    pub fn new(width: usize, height: usize) -> Mat {
        Mat {
            cols: height,
            contents: vec![0.0; width * height]
        }
    }

    /// Gets the value at the given coordinates
    pub fn get(&self, x: usize, y: usize) -> Option<f64> {
        self.contents.get(x * self.cols + y).cloned()
    }

    /// Sets the value at the given coordinates
    pub fn set(&mut self, x: usize, y: usize, value: f64) {
        self.contents[x * self.cols + y] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mat = Mat::new(1, 2);
        assert_eq!(mat.get(0, 1), Some(0.0));
        assert_eq!(mat.get(1, 1), None);
    }

    #[test]
    fn set() {
        let mut mat = Mat::new(1, 1);
        mat.set(0, 0, 4.20);
        assert_eq!(mat.get(0, 0), Some(4.20));
    }

    #[test]
    #[should_panic]
    fn set_oob() {
        let mut mat = Mat::new(1, 1);
        mat.set(1, 1, 4.20);
    }
}

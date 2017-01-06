#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidIndex
}

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
        self.contents.get(x * self.cols + y).map(|y| *y)
    }

    /// Sets the value at the given coordinates
    pub fn set(&mut self, x: usize, y: usize, value: f64) -> Result<(), Error> {
        let r = self.get_mut(x, y).ok_or(Error::InvalidIndex)?;
        *r = value;
        Ok(())
    }

    /// Gets a mutable reference to the value at the given coordinates
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut f64> {
        self.contents.get_mut(x * self.cols + y)
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
        assert_eq!(mat.set(0, 0, 4.20), Ok(()));
        assert_eq!(mat.set(1, 1, 4.20), Err(Error::InvalidIndex));
        assert_eq!(mat.get(0, 0), Some(4.20));
    }
}

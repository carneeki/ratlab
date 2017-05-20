use num_traits::Num;
use std::ops::{Index,IndexMut};

/// An r x c matrix of a numeric type.
///
/// # Examples
/// `TODO: write me`
#[derive(Debug)]
struct Matrix<T> {
  _cols: usize,
  _rows: usize,
  _data: Vec<T>,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<T: Num + Copy> Matrix<T> {
    /// Constructs a new, zero filled r x c matrix
    ///
    /// # Example:
    ///
    /// ```norun
    /// let mut m = Matrix::new(1,3);
    /// ```
    /// Should return a single row matrix, `m == (0  0  0)`
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix{
            _cols: cols,
            _rows: rows,
            _data: Vec::new(),
        }
    }
    /// Gets the number of columns in matrix
    pub fn cols(&self) -> usize { self._cols }

    /// Gets the number of rows in matrix
    pub fn rows(&self) -> usize { self._rows }

    /// Fills matrix with a given number
    pub fn fill_with(&mut self, v: T) -> &mut Matrix<T> {
        self._data = vec![v; (self.cols() * self.rows()) as usize];
        self
    }

    /// Converts the current matrix into an identity matrix.
    ///
    /// # Note:
    /// Matrix *must* be a square matrix or a panic will occur.
    pub fn identity(&mut self) -> &mut Matrix<T> {
        if self.is_square() == false {
            panic!("Identity matrices must be square.");
        }

        self.fill_with(T::zero());

        for i in 0 .. self.cols() {
            self[(i,i)] = T::one();
        }
        self
    }
}

trait Square { fn is_square(&self) -> bool; }
impl<T : Num + Copy> Square for Matrix<T> {
    fn is_square(&self) -> bool {
        self.cols() == self.rows()
    }
}

impl<T : Num + Copy> Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    /// Used to get element rc from a matrix where r is row, c is col
    /// with a zero based index.
    /// # Example:
    /// Given the following matrix:
    /// ```text
    ///     ( 1   1   2 )
    /// m = ( 9   9   9 )
    ///     ( 3   4   3 )
    /// ```
    /// `m[(1,3)]` will return 2
    fn index(&self, idx: (usize,usize)) -> &T {
        let addr = (idx.0*self.cols() + idx.1) as usize;
        &self._data[addr]
    }
}

impl<T : Num + Copy> IndexMut<(usize,usize)> for Matrix<T> {
    /// Used to get element rc from a matrix where r is row, c is col
    /// with a zero based index.
    /// # Example:
    /// Given the following matrix:
    /// ```text
    ///     ( 1   1   2 )
    /// m = ( 9   9   9 )
    ///     ( 3   4   3 )
    /// ```
    /// `m[(1,3)] = 4` will set the 2 to a 4
    fn index_mut(&mut self, idx: (usize,usize)) -> &mut T {
        let addr = (idx.0*self.cols() + idx.1) as usize;
        &mut self._data[addr]
    }
}

#[derive(Debug)]
struct MatrixBuilder<T> {
  _cols: usize,
  _rows: usize,
  _data: Vec<T>,
}

impl<T: Num + Copy> MatrixBuilder<T> {
    pub fn new() -> MatrixBuilder<T> {
        MatrixBuilder {
            _cols: 0 as usize,
            _rows: 0 as usize,
            _data: Vec::new(),
        }
    }

    pub fn set_cols(&mut self, cols: usize) -> &mut MatrixBuilder<T> {
        self._cols = cols;
        self
    }

    pub fn set_rows(&mut self, rows: usize) -> &mut MatrixBuilder<T> {
        self._rows = rows;
        self
    }

    pub fn finalize(&self) -> Matrix<T> {
        if self._cols == 0 {
            panic!("cols cannot be zero");
        }
        if self._rows == 0 {
            panic!("rows cannot be zero");
        }

        Matrix {
            _cols: self._cols,
            _rows: self._rows,
            _data: vec![T::zero(); (self._rows*self._cols)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_1x1() {
        let mut m = MatrixBuilder::new()
            .set_cols(1)
            .set_rows(1)
            .finalize();

        m._cols = 900;  // FIXME: This should have errored during compile

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);

        m._data[0] = 1; // FIXME: This should have errored during compile
                        // Should not be able to write to data
                        // how did these fields become public?
                        
                        // is it because the tests module is part of the matrix
                        // parent module?

        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);
        m[(0,0)] = 1;
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn one_1x1() {
        let mut m = MatrixBuilder::new()
            .set_cols(1)
            .set_rows(1)
            .finalize();
        m.fill_with(1);

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);
        m[(0,0)] = 0;
        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn identity_1x1() {
        let mut m = MatrixBuilder::new()
            .set_cols(1)
            .set_rows(1)
            .finalize();
        m.identity();

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);
        m[(0,0)] = 0;
        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn zero_2x2() {
        let mut m = MatrixBuilder::new()
            .set_cols(2)
            .set_rows(2)
            .finalize();

        assert_eq!(m.cols()  , 2 as usize);
        assert_eq!(m.rows()  , 2 as usize);

        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m[(0,1)]  , 0);
        assert_eq!(m[(1,0)]  , 0);
        assert_eq!(m[(1,1)]  , 0);
        assert_eq!(m._data[0], 0);
        assert_eq!(m._data[1], 0);
        assert_eq!(m._data[2], 0);
        assert_eq!(m._data[3], 0);

        m[(0,0)] = 1;
        m[(0,1)] = 1;
        m[(1,0)] = 1;
        m[(1,1)] = 1;

        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m[(0,1)]  , 1);
        assert_eq!(m[(1,0)]  , 1);
        assert_eq!(m[(1,1)]  , 1);
        assert_eq!(m._data[0], 1);
        assert_eq!(m._data[1], 1);
        assert_eq!(m._data[2], 1);
        assert_eq!(m._data[3], 1);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn one_2x2() {
        let mut m = MatrixBuilder::new()
            .set_cols(2)
            .set_rows(2)
            .finalize();
        m.fill_with(1);

        assert_eq!(m.cols()  , 2 as usize);
        assert_eq!(m.rows()  , 2 as usize);

        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m[(0,1)]  , 1);
        assert_eq!(m[(1,0)]  , 1);
        assert_eq!(m[(1,1)]  , 1);
        assert_eq!(m._data[0], 1);
        assert_eq!(m._data[1], 1);
        assert_eq!(m._data[2], 1);
        assert_eq!(m._data[3], 1);

        m[(0,0)] = 0;
        m[(0,1)] = 0;
        m[(1,0)] = 0;
        m[(1,1)] = 0;

        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m[(0,1)]  , 0);
        assert_eq!(m[(1,0)]  , 0);
        assert_eq!(m[(1,1)]  , 0);
        assert_eq!(m._data[0], 0);
        assert_eq!(m._data[1], 0);
        assert_eq!(m._data[2], 0);
        assert_eq!(m._data[3], 0);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn identity_2x2() {
        let mut m = MatrixBuilder::new()
            .set_cols(2)
            .set_rows(2)
            .finalize();
        m.identity();

        assert_eq!(m.cols()  , 2 as usize);
        assert_eq!(m.rows()  , 2 as usize);

        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m[(0,1)]  , 0);
        assert_eq!(m[(1,0)]  , 0);
        assert_eq!(m[(1,1)]  , 1);
        assert_eq!(m._data[0], 1);
        assert_eq!(m._data[1], 0);
        assert_eq!(m._data[2], 0);
        assert_eq!(m._data[3], 1);

        m[(0,0)] = 0;
        m[(0,1)] = 1;
        m[(1,0)] = 1;
        m[(1,1)] = 0;

        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m[(0,1)]  , 1);
        assert_eq!(m[(1,0)]  , 1);
        assert_eq!(m[(1,1)]  , 0);
        assert_eq!(m._data[0], 0);
        assert_eq!(m._data[1], 1);
        assert_eq!(m._data[2], 1);
        assert_eq!(m._data[3], 0);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn zero_1x2() {
        let mut m = MatrixBuilder::new()
            .set_cols(2)
            .set_rows(1)
            .finalize();
        m.fill_with(0);

        assert_eq!(m.cols() , 2 as usize);
        assert_eq!(m.rows() , 1 as usize);

        assert_eq!(m[(0,0)] , 0);
        assert_eq!(m[(0,1)] , 0);

        m[(0,0)] = 1;
        m[(0,1)] = 1;

        assert_eq!(m[(0,0)] , 1);
        assert_eq!(m[(0,1)] , 1);

        assert_eq!(m.is_square(), false);
    }
}

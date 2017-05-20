use num_traits::Num;
use std::ops::{Index,IndexMut};

#[derive(Debug)]
struct Matrix<T> {
  _cols: usize,
  _rows: usize,
  _data: Vec<T>,
}

impl<T: Num + Copy> Matrix<T> {
    pub fn cols(&self) -> usize { self._cols }
    pub fn rows(&self) -> usize { self._rows }

    pub fn fill_with(&mut self, v: T) -> &mut Matrix<T> {
        self._data = vec![v; (self.cols() * self.rows()) as usize];
        self
    }

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
    fn index(&self, idx: (usize,usize)) -> &T {
        let addr = (idx.0 + idx.1*self.cols()) as usize;
        &self._data[addr]
    }
}

impl<T : Num + Copy> IndexMut<(usize,usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize,usize)) -> &mut T {
        let addr = (idx.0 + idx.1*self.cols()) as usize;
        &mut self._data[addr]
    }
}

#[derive(Debug)]
struct MatrixBuilder<T>(Matrix<T>);

impl<T: Num + Copy> MatrixBuilder<T> {
    pub fn new() -> Matrix<T> {
        Matrix {
            _cols: 0 as usize,
            _rows: 0 as usize,
            _data: Vec::new(),
        }
    }

    pub fn set_cols(&mut self, cols: usize) -> &mut Matrix<T> {
        (self.0)._cols = cols;
        &mut self.0
    }

    pub fn set_rows(&mut self, rows: usize) -> &mut Matrix<T> {
        (self.0)._rows = rows;
        &mut self.0
    }

    pub fn finalize(self) -> Matrix<T> { // takeownership and destroy self
        if (self.0)._cols == 0 {
            panic!("cols cannot be zero");
        }
        if (self.0)._rows == 0 {
            panic!("rows cannot be zero");
        }

        Matrix {
            _cols: (self.0).cols(),
            _rows: (self.0).rows(),
            _data: (self.0)._data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_1x1() {
        let mut m = MatrixBuilder::new().set_cols(1).set_rows(1).will_with(0).finalize();

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);
        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);
        m[(0,0)] = 1;
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);

        assert_eq!(m.is_square(), true);
    }

    #[test]
    fn one_1x1() {
        let mut m = Matrix::new().cols(1).rows(1).fill_with(1);

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
        let mut m = Matrix::identity(1);

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
        let mut m = Matrix::fill_with(2,2,0);

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
        let mut m = Matrix::fill_with(2,2,1);

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
        let mut m = Matrix::new().set_cols(2).set_rows(2).identity().finalize();

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
        let mut m = Matrix::new().cols(2).rows(1).fill_with(0).finalize();

        assert_eq!(m.cols() , 1 as usize);
        assert_eq!(m.rows() , 2 as usize);

        assert_eq!(m[(0,0)] , 0);
        assert_eq!(m[(0,1)] , 0);

        m[(0,0)] = 1;
        m[(0,1)] = 1;

        assert_eq!(m[(0,0)] , 1);
        assert_eq!(m[(0,1)] , 1);

        assert_eq!(m.is_square(), false);
    }
}

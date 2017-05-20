use num_traits::Num;
use std::ops::{Index,IndexMut};

#[derive(Debug)]
struct Matrix<T> {
  _cols: usize,
  _rows: usize,
  _data: Vec<T>,
}

impl<T: Num + Copy> Matrix<T> {
    pub fn new(cols: usize, rows: usize) -> Matrix<T> {
        Matrix {
            _cols: cols,
            _rows: rows,
            _data: Vec::new(),
        }
    }

    pub fn cols(&self) -> usize { self._cols }
    pub fn rows(&self) -> usize { self._rows }

    pub fn fill_with(mut self, v: T) -> Matrix<T> {
        self._data = vec![v; (self.cols()*self.rows()) as usize];
        self
    }

    pub fn identity(mut self) -> Matrix<T> {
        if self.rows() != self.cols() {
            panic!("Matrix is not square. Bye");
        }

        self.fill_with(T::zero());
        let mut addr = 0;
        for i in 0 .. self.cols() {
            addr = (i + i*self.cols()) as usize;
            self._data[addr] = T::one();
        }
        self
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

trait Squareness { fn is_square(&self) -> bool; }
impl<T : Num + Copy> Squareness for Matrix<T> {
    fn is_square(&self) -> bool {
        self.cols() == self.rows()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_1x1() {
        let mut m = Matrix::new(1,1).fill_with(0);

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);
        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);
        m[(0,0)] = 1;
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);
    }

    #[test]
    fn one_1x1() {
        let mut m = Matrix::new(1,1).fill_with(1);

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);
        m[(0,0)] = 0;
        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);
    }

    #[test]
    fn identity_1x1() {
        let mut m = Matrix::new(1,1).identity();

        assert_eq!(m.cols()  , 1 as usize);
        assert_eq!(m.rows()  , 1 as usize);
        assert_eq!(m[(0,0)]  , 1);
        assert_eq!(m._data[0], 1);
        m[(0,0)] = 0;
        assert_eq!(m[(0,0)]  , 0);
        assert_eq!(m._data[0], 0);
    }

    #[test]
    fn zero_2x2() {
        let mut m = Matrix::new(2,2).fill_with(0);

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
    }

    #[test]
    fn one_2x2() {
        let mut m = Matrix::new(2,2).fill_with(1);

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
    }

    #[test]
    fn identity_2x2() {
        let mut m = Matrix::new(2,2).identity();

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
    }
}

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

    pub fn fill_with(cols: usize, rows: usize, v: T) -> Matrix<T> {
        Matrix {
            _cols: cols,
            _rows: rows,
            _data: vec![v; (cols * rows) as usize],
        }
    }

    pub fn identity(dim: usize) -> Matrix<T> {
        let mut m = Matrix {
            _cols: dim,
            _rows: dim,
            _data: vec![T::zero(); (dim * dim) as usize],
        };

        let mut addr = 0;
        for i in 0 .. dim {
            addr = (i + i * dim) as usize;
            m._data[addr] = T::one();
        }
        m
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

trait Square { fn is_square(&self) -> bool; }
impl<T : Num + Copy> Square for Matrix<T> {
    fn is_square(&self) -> bool {
        self.cols() == self.rows()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_1x1() {
        let mut m = Matrix::fill_with(1,1,0);

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
        let mut m = Matrix::fill_with(1,1,1);

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
        let mut m = Matrix::identity(2);

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
        let mut m = Matrix::fill_with(1, 2, 0);
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

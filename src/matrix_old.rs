struct MatrixOld<T> {
  _cols: usize,
  _rows: usize,
  _data: Vec<T>,
}

/*
impl<T> Matrix<T> {
    pub fn new(cols: usize, rows: usize) -> Matrix<T> {
        Matrix {
            _cols: cols,
            _rows: rows,
            _data: Vec::new(),
        }
    }

    pub fn cols(&self) -> usize { self._cols }
    pub fn rows(&self) -> usize { self._rows }

    pub fn fill_with(&mut self, v: <T>) -> &mut Matrix<T> {
        self._data = vec![v; (self.cols()*self.rows()) as usize];
        self
    }

    pub fn identity(&mut self) -> &mut Matrix<T> {
        if self.rows() != self.cols() {
            panic!("Matrix is not square. Bye");
        }

        self.fill_with(0);
        let mut addr = 0;
        for i in 0..self.cols() {
            addr = (i + i*self.cols()) as usize;
            self.data[addr] = 1;
            //self.set(i, i, 1);
        }
        /*
        let mut i = 0;
        while i < self._data.len()
        {
            self.data[i] = 1 as f64;
            i+= (self.cols() +1) as usize;
        }
        */
        self
    }

    pub fn finalize(&mut self) -> Matrix<T> {
        Matrix {
            _cols: self.cols(),
            _rows: self.rows(),
            _data: self._data.clone()
        }
    }
}

trait Squareness {
    fn is_square(&self) -> bool;
}
*/
/*
impl Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, idx: (usize,usize)) -> &<T> {
        let addr = (idx.0 + idx.1*self.cols()) as usize;
        self._data[addr]
    }
}

impl Index_Mut<(usize,usize)> for Matrix<T> {
    type Output = T;
    fn index(&mut self, idx: (usize,usize)) -> &mut <T> {
        let addr = (idx.0 + idx.1*self.cols()) as usize;
        self._data[addr]
    }
}
*/
/*
impl<T> Squareness for Matrix<T> {
    fn is_square(&self) -> bool {
        self.cols() == self.rows()
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn zero_1x1() {
        let mut m = Matrix::new(1,1).fill_with(0);
        assert_eq!(m.data.len(), 0);
        assert_eq!(m.data[0], 0);
    }
}
*/

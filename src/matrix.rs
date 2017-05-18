pub struct Matrix<T> {
  _cols: usize,
  _rows: usize,
  _data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(cols: usize, rows: usize) -> Matrix<T> {
        Matrix {
            _cols: cols,
            _rows: rows,
            _data: Vec::new(),
        }
    }

    pub fn cols(&mut self) -> usize { self._cols.clone() }
    pub fn rows(&mut self) -> usize { self._rows.clone() }

    pub fn get(&mut self, i: usize, j: usize) -> T {
        &mut self._data[(i+j*self.cols()) as usize]
    }
    pub fn set(&mut self, i: usize, j: usize, d: T) {
        self._data[(i+j*self.cols()) as usize] = d;
    }

    pub fn fill_with(&mut self, v: T) -> &mut Matrix<T> {
        self._data = vec![v; (self.cols()*self.rows()) as usize];
        self
    }
    pub fn identity(&mut self) -> &mut Matrix<T> {
        if self.rows() != self.cols() {
            panic!("Matrix is not square. Bye");
        }

        self.fill_with(0);
        //self._data = vec![0 as f64, (self.cols()*self.rows()) as f64];

        for i in 0..self.cols() {
            self.set(i, i, 1);
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

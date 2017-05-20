#[derive(Builder, Debug)]
#[builder(field(private))]
struct Matrix<T> {
  cols: usize,
  rows: usize,
  data: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_1x1() {
        let m = MatrixBuilder::default()
            .cols(3)
            .rows(3)
            .build();

        println!("{:?}", m);

        //assert_eq!(m.cols  , 3 as usize);
        assert_eq!(m.cols(), 3 as usize);
        //assert_eq!(m.rows  , 3 as usize);
        assert_eq!(m.rows(), 3 as usize);
        //assert_eq!(m.data.len(), 0);
        //assert_eq!(m.data[0], 0);
    }

}

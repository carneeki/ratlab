//! Matrices in Rust (sort of)
//!
//! # Example Types
//!
//! * Row vectors
//! * Column vectors
//! * 2x2, 3x3, to mxn matrices
//!
//! Matrices are stored internally as row-major order as a vector, however
//! they are addressed using column-major order by default (that is M_rc).
//! The 'getter' is to access the vector by way of a tuple. Suppose we have
//! the matrix `M` =
//! ```math
//! ┌      ┐
//! │ 4  3 │
//! │ 2  1 │
//! └      ┘
//! ```
//! The element '3' would be accessed by `M[(0,1)]`, and we could write a new
//! value to the position occupied by 2 with `M[(1,0)] = 0;`
//!
//! A MatrixBuilder has been provided to aid in building matrices which uses
//! `.set_cols(usize)` and `.set_rows(usize)` to set the dimensions.
//!
//! ## TODO List
//! The Matrix implementation will include functions for accessing row and
//! column vectors such that
//! `M` =
//! ```math
//! ┌      ┐
//! │ 4  3 │
//! │ 2  1 │
//! └      ┘
//! ```
//! could be accessed by way of `M.col(0)` would return a column vector:
//! ```math
//! ┌   ┐
//! │ 4 │
//! │ 2 │
//! └   ┘
//! ```
//! or `M.row(1)` would return a row vector:
//! ```math
//! ┌      ┐
//! │ 4  3 │
//! └      ┘
//! ```
//! or the two could be combined to access a single element, `m.col(1).row(1)`
//! ```math
//! ┌   ┐
//! │ 1 │
//! └   ┘
//! ```
//! A further refinement might be to take a count of the elements returned
//! prior to actually returning them, and if only a single value is returned,
//! return it as a single value rather than as a vector. This will require some
//! consideration to see if it is worthwhile. Methods such as
//! `M.get_rc<T>(row,col)` and `M.get_cr<T>(col,row)` might be more sensible.
//! concerning return types and ordering

extern crate num;
extern crate num_traits;

pub mod matrix;

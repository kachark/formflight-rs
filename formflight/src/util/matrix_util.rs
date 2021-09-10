use std::fmt;

use na::DMatrix;

#[derive(Debug, Clone)]
pub struct MatrixCompareError;

impl fmt::Display for MatrixCompareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix dimensions not consistent!")
    }
}

/// Extracts desired block from a matrix
pub fn block(start: (usize, usize), end: (usize, usize), matrix: &DMatrix<f32>) -> DMatrix<f32> {
    //TODO check for start and end valid for matrix

    let (_nrows, _ncols) = matrix.shape();

    let (dx, dy) = (end.0 - start.0, end.1 - start.1); // assume that start < end

    let mut work = DMatrix::<f32>::zeros(dx + 1, dy + 1);

    // iterate over matrix
    for i in start.0..end.0 + 1 {
        for j in start.1..end.1 + 1 {
            work[(i - start.0, j - start.1)] = matrix[(i, j)];
        }
    }

    work
}

/// Horizontally combines two matrices of similar dimensions
pub fn hcombine(
    lmatrix: &DMatrix<f32>,
    rmatrix: &DMatrix<f32>,
) -> Result<DMatrix<f32>, MatrixCompareError> {
    // Check for matching number of rows
    if lmatrix.shape().0 != rmatrix.shape().0 {
        return Err(MatrixCompareError);
    }

    // Allocate new matrix
    let rows = lmatrix.shape().0;
    let cols = lmatrix.shape().1 + rmatrix.shape().1;
    let mut concatenated = DMatrix::<f32>::zeros(rows, cols);

    for i in 0..lmatrix.shape().0 {
        for j in 0..lmatrix.shape().1 {
            concatenated[(i, j)] = lmatrix[(i, j)];
        }
    }

    let dy = lmatrix.shape().1;
    for i in 0..rmatrix.shape().0 {
        let mut j_rmatrix = 0;
        for j_shifted in dy..dy + rmatrix.shape().1 {
            concatenated[(i, j_shifted)] = rmatrix[(i, j_rmatrix)];
            j_rmatrix += 1;
        }
    }

    Ok(concatenated)
}

/// Vertically combines two matrices of similar dimensions
pub fn vcombine(
    umatrix: &DMatrix<f32>,
    bmatrix: &DMatrix<f32>,
) -> Result<DMatrix<f32>, MatrixCompareError> {
    // Check for matching number of rows
    if umatrix.shape().1 != bmatrix.shape().1 {
        return Err(MatrixCompareError);
    }

    // Allocate new matrix
    let rows = umatrix.shape().0 + bmatrix.shape().0;
    let cols = umatrix.shape().1;
    let mut concatenated = DMatrix::<f32>::zeros(rows, cols);

    for i in 0..umatrix.shape().0 {
        for j in 0..umatrix.shape().1 {
            concatenated[(i, j)] = umatrix[(i, j)];
        }
    }

    let dx = umatrix.shape().0;
    for j in 0..bmatrix.shape().1 {
        let mut i_bmatrix = 0;
        for i_shifted in dx..dx + bmatrix.shape().0 {
            concatenated[(i_shifted, j)] = bmatrix[(i_bmatrix, j)];
            i_bmatrix += 1;
        }
    }

    Ok(concatenated)
}

/// Prints matrix to console in an easy to read way
pub fn print_matrix(matrix: &DMatrix<f32>) {
    for i in 0..matrix.shape().0 {
        let mut row = vec![0.0; matrix.shape().1];
        for j in 0..matrix.shape().1 {
            row[j] = matrix[(i, j)];
        }
        println!("{:?}", row);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hconcat() {
        let lmatrix = DMatrix::from_row_slice(2, 2, &[0., 1., 0., 0.]);

        let rmatrix = DMatrix::from_row_slice(2, 1, &[0., 1.]);

        let concatenated = hcombine(&lmatrix, &rmatrix);

        // print_matrix(&lmatrix);
        // print_matrix(&rmatrix);
        // print_matrix(&concatenated.unwrap());

        let correct = DMatrix::from_row_slice(2, 3, &[0., 1., 0., 0., 0., 1.]);

        let _ = relative_eq!(concatenated.unwrap(), correct);
    }

    #[test]
    fn test_vconcat() {
        let umatrix = DMatrix::from_row_slice(2, 2, &[0., 1., 0., 0.]);

        let bmatrix = DMatrix::from_row_slice(1, 2, &[0., 1.]);

        let concatenated = vcombine(&umatrix, &bmatrix);

        // print_matrix(&umatrix);
        // print_matrix(&bmatrix);
        // print_matrix(&concatenated.unwrap());

        let correct = DMatrix::from_row_slice(3, 2, &[0., 1., 0., 0., 0., 1.]);

        let _ = relative_eq!(concatenated.unwrap(), correct);
    }

    #[test]
    fn test_block() {
        let matrix = DMatrix::from_row_slice(
            4,
            4,
            &[
                0., 1., 2., 3.,
                4., 5., 6., 7.,
                8., 9., 10., 11.,
                12., 13., 14., 15.,
            ],
        );

        let block = block((0, 1), (2, 2), &matrix);

        let block_true = DMatrix::from_row_slice(
            3, 2, 
            &[
                1., 2.,
                5., 6.,
                9., 10.
            ]
        );

        // println!("block: ");
        // print_matrix(&block);
        // println!("true: ");
        // print_matrix(&block_true);

        let _ = relative_eq!(block, block_true);
    }

}

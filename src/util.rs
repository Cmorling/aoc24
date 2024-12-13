use std::ops::ControlFlow;

pub struct TwoDWindowIterator<'a, T> {
    matrix: &'a [Vec<T>],
    window_size: (usize, usize),
    current_row: usize,
    current_col: usize,
    total_windows: usize,
}

impl<'a, T> TwoDWindowIterator<'a, T> {
    pub fn new(matrix: &'a [Vec<T>], window_size: (usize, usize)) -> Self {
        let (window_rows, window_cols) = window_size;
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };

        let total_windows = if rows >= window_rows && cols >= window_cols {
            (rows - window_rows + 1) * (cols - window_cols + 1)
        } else {
            0
        };

        TwoDWindowIterator {
            matrix,
            window_size,
            current_row: 0,
            current_col: 0,
            total_windows,
        }
    }
    pub fn len(&self) -> usize {
        self.total_windows
    }

    pub fn is_empty(&self) -> bool {
        self.total_windows == 0
    }
}

impl<T: Clone + Copy> Iterator for TwoDWindowIterator<'_, T> {
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (window_rows, window_cols) = self.window_size;
        let rows = self.matrix.len();

        if rows == 0 || self.matrix[0].is_empty() {
            return None;
        }

        let cols = self.matrix[0].len();

        if self.current_row + window_rows > rows {
            return None;
        }

        if self.current_col + window_cols > cols {
            self.current_row += 1;
            self.current_col = 0;

            if self.current_row + window_rows > rows {
                return None;
            }
        }

        let mut window = Vec::new();
        for i in 0..window_rows {
            let row_slice = &self.matrix[self.current_row + i];
            let row_window = row_slice[self.current_col..(self.current_col + window_cols)].to_vec();
            window.push(row_window);
        }

        self.current_col += 1;

        Some(window)
    }
}

pub fn matrix_transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut t_matrix = vec![vec![]; matrix[0].len()];

    matrix.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(ind, value)| {
            t_matrix[ind].push(value.clone());
        });
    });
    t_matrix
}
pub fn matrix_count_row_match<T: Clone + std::cmp::PartialEq>(
    matrix: &[Vec<T>],
    pattern: &[T],
) -> usize {
    matrix
        .iter()
        .filter(|group| group.starts_with(pattern))
        .count()
}

pub fn has_common_element<T: PartialEq>(vec_a: &[T], vec_b: &[T]) -> bool {
    vec_a.iter().any(|item| vec_b.contains(item))
}

pub fn find_first_common_index<T: PartialEq>(vec_a: &[T], vec_b: &[T]) -> Option<(usize, usize)> {
    match vec_a
        .iter()
        .enumerate()
        .try_for_each(|(i, a)| match vec_b.iter().position(|b| b == a) {
            Some(j) => ControlFlow::Break((i, j)),
            None => ControlFlow::Continue(()),
        }) {
        ControlFlow::Break((i, j)) => Some((i, j)),
        ControlFlow::Continue(()) => None,
    }
}

pub fn two_d_window<T: Clone>(matrix: &[Vec<T>], size: (usize, usize)) -> Vec<Vec<Vec<T>>> {
    let (window_rows, window_cols) = size;
    let m_x = matrix.len();
    let m_y = matrix[0].len();

    let mut windows = Vec::new();

    (0..=(m_x - window_rows))
        .for_each(|i| {
            (0..=(m_y - window_cols))
                .for_each(|j| {
                   let mut window = Vec::new(); 
                   (0..window_rows)
                       .for_each(|k| {
                            window.push(matrix[i + k][j..(j + window_cols)].to_vec());
                       });
                   windows.push(window);
                })
        });
    windows
}

pub fn matrix_transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    
    let mut t_matrix = vec![vec![]; matrix[0].len()];
    
    matrix
        .iter()
        .for_each(|row| {
            row
                .iter()
                .enumerate()
                .for_each(|(ind, value)| {
                    t_matrix[ind].push(value.clone());
                });
        });
    t_matrix
}
pub fn matrix_count_row_match<T: Clone + std::cmp::PartialEq>(matrix: &[Vec<T>], pattern: &[T]) -> usize {
    matrix
        .iter()
        .filter(|group| {
            group.starts_with(&pattern)
        })
        .count()
}

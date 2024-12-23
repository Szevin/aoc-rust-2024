advent_of_code::solution!(4);

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![];
    for i in 0..matrix[0].len() {
        let mut row = vec![];
        for j in 0..matrix.len() {
            row.push(matrix[j][i]);
        }
        transposed.push(row);
    }
    transposed
}

fn count_substrings(haystack: &Vec<char>, needle: &str) -> usize {
    let forwards = haystack
        .windows(needle.len())
        .filter(|window| window.iter().collect::<String>() == needle)
        .count();

    let mut reverse_haystack = haystack.clone();
    reverse_haystack.reverse();
    let backwards = reverse_haystack
        .windows(needle.len())
        .filter(|window| window.iter().collect::<String>() == needle)
        .count();

    forwards + backwards
}

fn diagonal(matrix: &Vec<Vec<char>>) -> Vec<Vec<&char>> {
    let mut diagonals = vec![];

    for i in 0..matrix.len() {
        let mut diagonal_l = vec![];
        let mut diagonal_r = vec![];

        for j in 0..matrix.len() {
            if i + j < matrix.len() {
                diagonal_l.push(&matrix[i + j][j]);

                if i != 0 {
                    diagonal_r.push(&matrix[j][i + j]);
                }
            }
        }
        diagonals.push(diagonal_l);
        diagonals.push(diagonal_r);
    }

    diagonals
}

pub fn part_one(input: &str) -> Option<u32> {
    let xmas = "XMAS";

    let mut count = 0;

    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut cols = transpose(&rows);

    let rows_xmas_count = rows
        .iter()
        .map(|row| count_substrings(row, xmas))
        .sum::<usize>();
    let cols_xmas_count = cols
        .iter()
        .map(|col| count_substrings(col, xmas))
        .sum::<usize>();

    count += rows_xmas_count;
    count += cols_xmas_count;

    let diagonal_pos = diagonal(&rows)
        .iter()
        .map(|diagonal| diagonal.iter().map(|pos| **pos).collect())
        .collect::<Vec<Vec<char>>>();
    cols.reverse();
    let diagonal_neg = diagonal(&cols)
        .iter()
        .map(|diagonal| diagonal.iter().map(|pos| **pos).collect())
        .collect::<Vec<Vec<char>>>();

    let diagonal_pos_xmas_count = diagonal_pos
        .iter()
        .map(|diagonal| count_substrings(diagonal, xmas))
        .sum::<usize>();
    let diagonal_neg_xmas_count = diagonal_neg
        .iter()
        .map(|diagonal| count_substrings(diagonal, xmas))
        .sum::<usize>();

    count += diagonal_pos_xmas_count;
    count += diagonal_neg_xmas_count;

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;

    let coords_of_a = matrix
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            let a_vec = row
                .iter()
                .enumerate()
                .filter_map(|(j, &c)| {
                    if c == 'A'
                        && (1..matrix.len() - 1).contains(&i)
                        && (1..row.len() - 1).contains(&j)
                    {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>();

            if a_vec.len() > 0 {
                Some(a_vec)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    for (y, x) in coords_of_a {
        let valid_words = vec!["MSSM", "SMMS", "SSMM", "MMSS"];

        let diagonal = String::from_iter(vec![
            matrix[y - 1][x - 1],
            matrix[y - 1][x + 1],
            matrix[y + 1][x + 1],
            matrix[y + 1][x - 1],
        ]);

        if valid_words.contains(&diagonal.as_str()) {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

fn handle_top(trees: Vec<Vec<u32>>, mut flags: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for column_index in 0..trees.len() {
        let mut column: Vec<u32> = Vec::new();
        for row_index in 0..trees.len() {
            if row_index == 0 || trees[row_index][column_index] > column[row_index - 1] {
                flags[row_index][column_index] = true;
                column.push(trees[row_index][column_index])
            } else {
                column.push(column[row_index - 1])
            }
        }
    }
    return flags;
}

fn handle_bottom(trees: Vec<Vec<u32>>, mut flags: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for column_index in 0..trees.len() {
        let mut column: Vec<u32> = Vec::new();
        for row_index in 0..trees.len() {
            if row_index == 0 || trees[trees.len() - row_index - 1][column_index] > column[row_index - 1] {
                
                flags[trees.len() - row_index - 1][column_index] = true;
                column.push(trees[trees.len() - row_index - 1][column_index])
            } else {
                column.push(column[row_index - 1])
            }
        }
    }
    return flags;
}

fn handle_left(trees: Vec<Vec<u32>>, mut flags: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row_index in 0..trees.len() {
        let mut row: Vec<u32> = Vec::new();
        for column_index in 0..trees.len() {
            if column_index == 0 || trees[row_index][column_index] > row[column_index - 1] {
                flags[row_index][column_index] = true;
                row.push(trees[row_index][column_index])
            } else {
                row.push(row[column_index - 1])
            }
        }
    }
    return flags;
}

fn handle_right(trees: Vec<Vec<u32>>, mut flags: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row_index in 0..trees.len() {
        let mut row: Vec<u32> = Vec::new();
        for column_index in 0..trees.len() {
            if column_index == 0 || trees[row_index][trees.len() - column_index - 1] > row[column_index - 1] {
                flags[row_index][trees.len() - column_index - 1] = true;
                row.push(trees[row_index][trees.len() - column_index - 1])
            } else {
                row.push(row[column_index - 1])
            }
        }
    }
    return flags;
}

fn distance_top(trees: Vec<Vec<u32>>, mut distances: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row_index in 0..trees.len() {
        for column_index in 0..trees.len() {
            let mut temp_row = row_index;
            let mut distance = 1;
            while temp_row != 0 && trees[temp_row - 1][column_index] < trees[row_index][column_index] {
                distance += 1;
                temp_row -= 1;
            }
            if temp_row == 0 {
                distance -= 1;
            }
            distances[row_index][column_index] *= distance;
        }
    }
    return distances;
}

fn distance_bottom(trees: Vec<Vec<u32>>, mut distances: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row_index in 0..trees.len() {
        for column_index in 0..trees.len() {
            let mut temp_row = row_index;
            let mut distance = 1;
            while temp_row < trees.len() - 1 && trees[temp_row + 1][column_index] < trees[row_index][column_index] {
                distance += 1;
                temp_row += 1;
            }
            if temp_row == trees.len() - 1 {
                distance -= 1;
            }
            distances[row_index][column_index] *= distance;
        }
    }
    return distances;
}

fn distance_left(trees: Vec<Vec<u32>>, mut distances: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row_index in 0..trees.len() {
        for column_index in 0..trees.len() {
            let mut temp_column = column_index;
            let mut distance = 1;
            while temp_column != 0 && trees[row_index][temp_column - 1] < trees[row_index][column_index] {
                distance += 1;
                temp_column -= 1;
            }
            if temp_column == 0 {
                distance -= 1;
            }
            distances[row_index][column_index] *= distance;
        }
    }
    return distances;
}

fn distance_right(trees: Vec<Vec<u32>>, mut distances: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row_index in 0..trees.len() {
        for column_index in 0..trees.len() {
            let mut temp_column = column_index;
            let mut distance = 1;
            while temp_column < trees.len() - 1 && trees[row_index][temp_column + 1] < trees[row_index][column_index] {
                distance += 1;
                temp_column += 1;
            }
            if temp_column == trees.len() - 1 {
                distance -= 1;
            }
            distances[row_index][column_index] *= distance;
        }
    }
    return distances;
}

pub fn solve8(input: String) -> (String, String) {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        trees.push(row);
    }
    let mut flags = vec![vec![false; trees.len()].clone(); trees.len()];

    flags = handle_top(trees.clone(), flags);
    flags = handle_bottom(trees.clone(), flags);
    flags = handle_left(trees.clone(), flags);
    flags = handle_right(trees.clone(), flags);

    let part1: i32 = flags
    .iter()
    .map(
        |vec| vec.iter().filter(|&&item| item).collect::<Vec<&bool>>().len() as i32
    )
    .sum();

    let mut distances = vec![vec![1; trees.len()].clone(); trees.len()];
    
    distances = distance_top(trees.clone(), distances);
    distances = distance_bottom(trees.clone(), distances);
    distances = distance_left(trees.clone(), distances);
    distances = distance_right(trees.clone(), distances);

    let part2 = distances.iter().map(|vec| vec.iter().max().unwrap()).max().unwrap();

    return (part1.to_string(), part2.to_string());
}
use std::io;
use rand::Rng;
use std::process::Command;

fn main() {
    let mut board:Vec<Vec<u32>> = vec![vec![0; 4]; 4];
    
    for _i in 0..2 {
        spawn_new_block(&mut board);
    }

    while !win_condition(&mut board) && playable_move_exists(&board) {
        let mut user_input = String::new();
        while user_input != "a" && user_input != "w" && user_input != "s" && user_input != "d" && user_input != "q" {
            cls();
            user_input.clear();
            show(&mut board);
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            user_input = user_input.trim().to_string();
        }
        if user_input == "q" {
            break;
        }
        update(user_input, &mut board);
    }
    cls();
    show(&mut board);
    match win_condition(&mut board) {
        true => print!("YOU WIN"),
        false => print!("YOU LOSE"),
    }
}

fn cls() {
    if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

fn show(vec: &Vec<Vec<u32>>) {
    for row in vec {
        for &element in row {
            print!("{:^4}", element);
        }
        println!();
    }
}

fn update(input_direction: String, board: &mut Vec<Vec<u32>>) {
    let mut changes = 1;
    let mut has_changed = false;
    let mut changed_tile: Vec<Vec<i32>> = vec![vec![0; 0]; 0];
    while changes != 0 {
        changes = 0;
        let mut direction_mod_x = 0;
        let mut direction_mod_y = 0;
        let mut i_loop_direction:Vec<isize> = vec![0, board.len() as isize, 1];
        let mut j_loop_direction:Vec<isize> = vec![0, board.len() as isize, 1];

        match input_direction.as_str() {
            "a" => {
                direction_mod_x = -1;
                j_loop_direction = vec![0, board.len() as isize, 1];
            },
            "d" => {
                direction_mod_x = 1;
                j_loop_direction = vec![board.len() as isize - 1, -1, -1];
            },
            "w" => {
                direction_mod_y = -1;
                i_loop_direction = vec![0, board.len() as isize, 1];
            },
            "s" => {
                direction_mod_y = 1;
                i_loop_direction = vec![board.len() as isize - 1, -1, -1];
            },
            _ => panic!("Oh no update() match got a wrong input_direction"),
        }
        
        let mut i = i_loop_direction[0];
        while i != i_loop_direction[1] {
            let mut j = j_loop_direction[0];
            while j != j_loop_direction[1] {
                if board[i as usize][j as usize] > 0 && is_inside_board(i, j, &input_direction, board.len() as isize)
                && board[(i + direction_mod_y) as usize][(j + direction_mod_x) as usize] == 0 {
                    board[(i + direction_mod_y) as usize][(j + direction_mod_x) as usize] = board[i as usize][j as usize];
                    board[i as usize][j as usize] = 0;
                    changes += 1;
                } else if board[i as usize][j as usize] > 0 && is_inside_board(i, j, &input_direction, board.len() as isize)
                && board[(i + direction_mod_y) as usize][(j + direction_mod_x) as usize] == board[i as usize][j as usize]
                && is_inside(&vec![(i + direction_mod_y) as i32, (j + direction_mod_x) as i32], &changed_tile) == false 
                && is_inside(&vec![i as i32, j as i32], &changed_tile) == false {
                    changed_tile.push(vec![(i + direction_mod_y) as i32, (j + direction_mod_x) as i32]);
                    board[(i + direction_mod_y) as usize][(j + direction_mod_x) as usize] *= 2;
                    board[i as usize][j as usize] = 0;
                    changes += 1;
                }
                j += j_loop_direction[2];
            }
            i += i_loop_direction[2];
        }
        if changes > 0 {
            has_changed = true;
        }
    }
    if has_changed {
        spawn_new_block(board);
    }
}

fn is_inside_board(i: isize, j: isize, input_direction: &String, board_l: isize) -> bool {
    match input_direction.as_str() {
        "a" => j - 1 >= 0,
        "d" => j + 1 < board_l,
        "w" => i - 1 >= 0,
        "s" => i + 1 < board_l,
        _ => false,
    }
}

fn is_inside(coordinates: &Vec<i32>, vec: &Vec<Vec<i32>>) -> bool {
    vec.iter().any(|element| element == coordinates)
}

fn spawn_new_block(board: &mut Vec<Vec<u32>>) {
    let legal_spawn_vector: Vec<(usize, usize)> = (0..board.len())
        .flat_map(|row| (0..board[row].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| board[row][col] == 0)
        .collect();

    let random_index = rand::thread_rng().gen_range(0..legal_spawn_vector.len());
    let (new_row, new_col) = legal_spawn_vector[random_index];
    let value = if rand::thread_rng().gen_range(1..=10) == 1 { 4 } else { 2 };

    board[new_row][new_col] = value;
}

fn win_condition(vec: &Vec<Vec<u32>>) -> bool {
    vec.iter().any(|row| row.iter().any(|&element| element >= 2048))
}

fn has_compatible_neighbours(coordinates: &Vec<usize>, vec: &Vec<Vec<u32>>) -> bool {
    if coordinates[0] > 0 && vec[coordinates[0]][coordinates[1]] == vec[coordinates[0]-1][coordinates[1]]
    || coordinates[0] < vec.len() - 1 && vec[coordinates[0]][coordinates[1]] == vec[coordinates[0]+1][coordinates[1]]
    || coordinates[1] > 0 && vec[coordinates[0]][coordinates[1]] == vec[coordinates[0]][coordinates[1]-1]
    || coordinates[1] < vec.len() - 1 && vec[coordinates[0]][coordinates[1]] == vec[coordinates[0]][coordinates[1]+1]
    || coordinates[0] > 0 && vec[coordinates[0]-1][coordinates[1]] == 0
    || coordinates[0] < vec.len() - 1 && vec[coordinates[0]+1][coordinates[1]] == 0
    || coordinates[1] > 0 && vec[coordinates[0]][coordinates[1]-1] == 0
    || coordinates[1] < vec.len() - 1 && vec[coordinates[0]][coordinates[1]+1] == 0 {
        return true;
    }
    return false;
}

fn playable_move_exists(vec: &Vec<Vec<u32>>) -> bool {
    vec.iter().enumerate().any(|(i, row)| row.iter().enumerate().any(|(j, _)| has_compatible_neighbours(&vec![i, j], vec)))
}
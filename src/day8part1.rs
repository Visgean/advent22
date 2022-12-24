use std::fs;

pub fn count_visible(forrest: &Vec<Vec<bool>>) -> usize {
    let mut count: usize = 0;

    for row in forrest {
        for tree in row {
            if *tree {
                count += 1;
            }
        }
    }
    count
}


pub fn print_visible(forrest: &Vec<Vec<bool>>) {
    println!();

    for row in forrest {
        for tree in row {
            if *tree {
                print!("🌲",)
            } else {
                print!("🧝")
            }
        }
        println!()
    }
}

pub fn day8() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day8").expect("Where file");

    let mut forrest: Vec<Vec<i8>> = vec![];
    let mut visible: Vec<Vec<bool>> = vec![];

    for line in contents.lines() {
        visible.push(vec![false; line.len()]);
        forrest.push(line.chars().into_iter().map(|x| x as i8).collect());
    }

    let dim_y = forrest.len();
    let dim_x = forrest.first()?.len();

    // left
    for y in 0..dim_y {
        let mut max_tree: i8 = -1;
        for x in 0..dim_x {
            let tree = forrest[y][x];
            if tree > max_tree {
                visible[y][x] = true;
                max_tree = tree;
                if max_tree == 9 {
                    break;
                }
            }
        }
    }

    // right
    for y in 0..dim_y {
        let mut max_tree: i8 = -1;
        for x in (0..dim_x).rev() {
            let tree = forrest[y][x];
            if tree > max_tree {
                visible[y][x] = true;
                max_tree = tree;
                if max_tree == 9 {
                    break;
                }
            }
        }
    }

    // up
    for x in 0..dim_x {
        let mut max_tree: i8 = -1;
        for y in 0..dim_y {
            let tree = forrest[y][x];

            if tree > max_tree {
                visible[y][x] = true;
                max_tree = tree;
                if max_tree == 9 {
                    break;
                }
            }
        }
    }

    // down
    for x in 0..dim_x {
        let mut max_tree: i8 = -1;
        for y in (0..dim_y).rev() {
            let tree = forrest[y][x];

            if tree > max_tree {
                visible[y][x] = true;
                max_tree = tree;
                if max_tree == 9 {
                    break;
                }
            }
        }
    }


    print_visible(&visible);

    println!("{}", count_visible(&visible));
    Some(0)
}

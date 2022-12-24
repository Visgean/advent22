use std::fs;


const RADIX: u32 = 10;

pub fn print_score(forrest: &Vec<Vec<usize>>) {
    println!();


    // for row in forrest {
    //     for tree in row {
    //         print!("{} ", tree)
    //     }
    //     println!()
    // }
    let max = forrest.iter().map(|x| x.iter().max().unwrap()).max().unwrap();
    println!("{}", max)
}


pub fn day8() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day8").expect("Where file");

    let mut forrest: Vec<Vec<u32>> = vec![];
    let mut score: Vec<Vec<usize>> = vec![];

    for line in contents.lines() {
        score.push(vec![1; line.len()]);
        forrest.push(line.chars().into_iter().map(|x| x.to_digit(RADIX).unwrap()).collect());
    }

    let dim_y = forrest.len();
    let dim_x = forrest.first()?.len();

    // left

    for y in 0..dim_y {
        for x in 0..dim_x {
            let tree = forrest[y][x];
            let mut view_score = 0;

            for x2 in (x + 1)..dim_x {
                let viewed_tree = forrest[y][x2];
                view_score += 1;
                if tree <= viewed_tree {
                    break
                }
            }
            score[y][x] *= view_score;
        }
    }


    // // right
    for y in 0..dim_y {
        for x in (0..dim_x).rev() {
            let tree = forrest[y][x];
            let mut view_score = 0;
            for x2 in (0..x).rev() {
                let viewed_tree = forrest[y][x2];
                view_score += 1;
                if tree <= viewed_tree {
                    break
                }
            }
            score[y][x] *= view_score;
        }
    }


    // up
    for x in 0..dim_x {
        for y in 0..dim_y {
            let tree = forrest[y][x];
            let mut view_score = 0;
            for y2 in (y + 1)..dim_y {
                let viewed_tree = forrest[y2][x];
                view_score += 1;
                if tree <= viewed_tree {
                    break
                }
            }
            score[y][x] *= view_score;
        }
    }


    // down - up
    for x in 0..dim_x {
        for y in 0..dim_y {
            let tree = forrest[y][x];
            let mut view_score = 0;
            // println!("{} ", tree);

            for y2 in (0..y).rev() {
                let viewed_tree = forrest[y2][x];
                view_score += 1;
                if tree <= viewed_tree {
                    break
                }
            }
            score[y][x] *= view_score;
        }
    }
    
    print_score(&score);
    Some(0)
}

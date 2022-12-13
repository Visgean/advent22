use std::fs;


pub fn day1() {
    let contents = fs::read_to_string("inputs/day1").expect("cant read day 1");

    let mut vals =  Vec::new();

    let mut curr_value = 0;
    let mut current_index = 0;

    for line in contents.lines(){
        if line == "" {
            vals.push(curr_value);

            curr_value = 0;
            current_index += 1;
            continue
        }

        let line_val: i32 = line.parse().unwrap();
        curr_value += line_val;
    }

    vals.sort();
    vals.reverse();

    let sum3: i32 = vals[0..3].iter().sum();


    println!("{:?}", sum3)

}
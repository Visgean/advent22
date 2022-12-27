enum Operation {
    Mult,
    Addi,
    Power2,
}

impl Operation {
    pub fn solve(&self, item: usize, param: usize) -> usize {
        match self {
            Operation::Mult => item * param,
            Operation::Addi => item + param,
            Operation::Power2 => item * item,
        }
    }
}

pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    op_parameter: usize,
    divisible_by: usize,
    throw_true: usize,
    throw_false: usize,
}





pub fn day11() -> Option<isize> {
    const MONKEY_COUNT: usize = 8;
    let mut monkeys = [
        // Monkey {
        //     items: vec![79, 98],
        //     operation: Operation::Mult,
        //     op_parameter: 19,
        //     divisible_by: 23,
        //     throw_true: 2,
        //     throw_false: 3,
        // },
        // Monkey {
        //     items: vec![54, 65, 75, 74],
        //     operation: Operation::Addi,
        //     op_parameter: 6,
        //     divisible_by: 19,
        //     throw_true: 2,
        //     throw_false: 0,
        // },
        // Monkey {
        //     items: vec![79, 60, 97],
        //     operation: Operation::Power2,
        //     op_parameter: 0,
        //     divisible_by: 13,
        //     throw_true: 1,
        //     throw_false: 3,
        // },
        // Monkey {
        //     items: vec![74],
        //     operation: Operation::Addi,
        //     op_parameter: 3,
        //     divisible_by: 17,
        //     throw_true: 0,
        //     throw_false: 1,
        // },

        // real input:
        Monkey {
            items: vec![91, 66],
            operation: Operation::Mult,
            op_parameter: 13,
            divisible_by: 19,
            throw_true: 6,
            throw_false: 2,
        },
        Monkey {
            items: vec![78, 97, 59],
            operation: Operation::Addi,
            op_parameter: 7,
            divisible_by: 5,
            throw_true: 0,
            throw_false: 3,
        },
        Monkey {
            items: vec![57, 59, 97, 84, 72, 83, 56, 76],
            operation: Operation::Addi,
            op_parameter: 6,
            divisible_by: 11,
            throw_true: 5,
            throw_false: 7,
        },
        Monkey {
            items: vec![81, 78, 70, 58, 84],
            operation: Operation::Addi,
            op_parameter: 5,
            divisible_by: 17,
            throw_true: 6,
            throw_false: 0,
        },
        Monkey {
            items: vec![60],
            operation: Operation::Addi,
            op_parameter: 8,
            divisible_by: 7,
            throw_true: 1,
            throw_false: 3,
        },
        Monkey {
            items: vec![57, 69, 63, 75, 62, 77, 72],
            operation: Operation::Mult,
            op_parameter: 5,
            divisible_by: 13,
            throw_true: 7,
            throw_false: 4,
        },
        Monkey {
            items: vec![73, 66, 86, 79, 98, 87],
            operation: Operation::Power2,
            op_parameter: 0,
            divisible_by: 3,
            throw_true: 5,
            throw_false: 2,
        },
        Monkey {
            items: vec![95, 89, 63, 67],
            operation: Operation::Addi,
            op_parameter: 2,
            divisible_by: 2,
            throw_true: 1,
            throw_false: 4,
        },
    ];



    let mut inspected = [0; MONKEY_COUNT];

    let mut LCD: usize = 1;
    for monkey_id in 0..MONKEY_COUNT {
        LCD *= monkeys[monkey_id].divisible_by;
    }

    for i in 0..10000 {
        println!("Round {i}");
        for monkey_id in 0..MONKEY_COUNT {
            println!("Monkey {monkey_id} {:?}", monkeys[monkey_id].items)
        }
        println!();

        for monkey_id in 0..MONKEY_COUNT {

            for _item_id in 0..monkeys[monkey_id].items.len() {
                inspected[monkey_id] += 1;

                let item = monkeys[monkey_id].items.remove(0);
                let divisible_by = monkeys[monkey_id].divisible_by;
                let op_param = monkeys[monkey_id].op_parameter;

                let level_bored = monkeys[monkey_id]
                    .operation
                    .solve(item, op_param) % LCD;

                let mut target_monkey = monkeys[monkey_id].throw_true;
                if level_bored % divisible_by == 0 {
                } else {
                    target_monkey = monkeys[monkey_id].throw_false;
                }
                monkeys[target_monkey].items.push(level_bored);
            }
        }
    }
    inspected.sort();
    println!("inspection counts: {:?}", inspected);


    Some(0)
}

use std::fs;

fn parse_lines(parsed_text: &str) -> u32 {
    let lines = parsed_text.lines();
    let mut sum = 0;
    for line in lines {
        let mut first_digit:Option<u32> = None;
        let mut last_digit:Option<u32> = None;
        // let mut i = 0;
        let mut i: i32 = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                let cur_digit = char.to_digit(10);
                if first_digit.is_none(){
                    first_digit = cur_digit;
                    // println!("First digit found: {:?}", first_digit);
                }
                last_digit = cur_digit;

            }
            i = i + 1;
        }
        if first_digit.is_some() && last_digit.is_some(){
            sum += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
    }
    sum
}

pub fn solve_question(){
    println!("parsing file in /inputs/q1.txt");
    let txt_file = fs::read_to_string("inputs/q1.txt").expect("File failed to load");
    let final_sum = parse_lines(&txt_file);
    println!("{}", final_sum);
}

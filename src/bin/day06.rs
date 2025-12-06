use anyhow::{Ok, Result};
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(6))?;
    let mut worksheet = WorkSheet::new();
    worksheet.parse_file(input);

    println!("Part 1: {}", part1(&mut worksheet)?);
    println!("Part 2: {}", part2(&mut worksheet)?);
    Ok(())
}

fn part1(worksheet: &mut WorkSheet) -> Result<i128> {
    let mut answer = 0;
    worksheet.calculate_answer_part_one();
    for column in &worksheet.columns {
        answer += column.answer;
    }

    Ok(answer)
}

fn part2(worksheet: &mut WorkSheet) -> Result<i128> {
    Ok(worksheet.calculate_answer_part_two())
}

struct WorkSheet {
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl WorkSheet {
    fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: vec![Row::new(); 5],
        }
    }
    pub fn calculate_answer_part_one(&mut self) {
        for column in &mut self.columns {
            column.calculate_answer_part_one();
        }
    }

    pub fn calculate_answer_part_two(&mut self) -> i128 {
        let mut sum: i128 = 0;
        if self.rows.is_empty() {
            return 0;
        }

        let line_length = self.rows[0].characters.len();
        let mut operator = ' ';
        let mut numbers: Vec<i128> = Vec::new();

        for i in (0..line_length).rev() {
            let c0 = *self.rows[0].characters.get(i).unwrap_or(&' ');
            let c1 = *self.rows[1].characters.get(i).unwrap_or(&' ');
            let c2 = *self.rows[2].characters.get(i).unwrap_or(&' ');
            let c3 = *self.rows[3].characters.get(i).unwrap_or(&' ');
            let cop = *self.rows[4].characters.get(i).unwrap_or(&' ');

            if operator == ' ' && (cop == '+' || cop == '*') {
                operator = cop;
            }

            let has_digit = [c0, c1, c2, c3].iter().any(|c| c.is_ascii_digit());

            if has_digit {
                let number_str: String = [c0, c1, c2, c3].iter().collect();
                let number_str = number_str.trim();
                if !number_str.is_empty() {
                    let number = number_str.parse::<i128>().unwrap();
                    numbers.push(number);
                }
            } else {
                if !numbers.is_empty() {
                    let local_sum = match operator {
                        '*' => numbers.iter().copied().product::<i128>(),
                        '+' => numbers.iter().copied().sum::<i128>(),
                        _ => {
                            println!("Oops: missing operator");
                            0
                        }
                    };
                    sum += local_sum;
                    numbers.clear();
                    operator = ' ';
                }
            }
        }

        //Don't forget the last columns
        if !numbers.is_empty() {
            let local_sum = match operator {
                '*' => numbers.iter().copied().product::<i128>(),
                '+' => numbers.iter().copied().sum::<i128>(),
                _ => {
                    println!("Oops: missing operator at end");
                    0
                }
            };
            sum += local_sum;
        }

        sum
    }

    pub fn parse_file(&mut self, input: String) {
        for (i, line) in lines(&input).enumerate() {
            self.rows[i].characters = line.chars().collect();
            //Operators
            if i == 4 {
                let operators: Vec<String> = line
                    .split_whitespace()
                    .map(|s| s.parse::<String>().unwrap())
                    .collect();

                for (y, operator) in operators.iter().enumerate() {
                    self.columns[y].operator = operator.to_string();
                }
            } else {
                let numbers: Vec<i128> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i128>().unwrap())
                    .collect();

                if self.columns.is_empty() {
                    self.columns = (0..numbers.len()).map(|_| Column::new()).collect()
                }

                for (y, number) in numbers.iter().enumerate() {
                    match i {
                        0 => self.columns[y].first_number = *number,
                        1 => self.columns[y].second_number = *number,
                        2 => self.columns[y].third_number = *number,
                        3 => self.columns[y].fourth_number = *number,
                        _ => println!("Hmm"),
                    }
                }
            }
        }
    }
}
struct Column {
    first_number: i128,
    second_number: i128,
    third_number: i128,
    fourth_number: i128,
    operator: String,
    answer: i128,
}

impl Column {
    fn new() -> Self {
        Self {
            first_number: 0,
            second_number: 0,
            third_number: 0,
            fourth_number: 0,
            operator: String::new(),
            answer: 0,
        }
    }

    fn calculate_answer_part_one(&mut self) {
        match self.operator.as_str() {
            "*" => {
                self.answer =
                    self.first_number * self.second_number * self.third_number * self.fourth_number
            }
            "+" => {
                self.answer =
                    self.first_number + self.second_number + self.third_number + self.fourth_number
            }

            _ => println!("Hmm"),
        }
    }
}

#[derive(Clone)]
struct Row {
    characters: Vec<char>,
}

impl Row {
    fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }
}

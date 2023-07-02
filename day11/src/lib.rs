use std::collections::VecDeque;

use nom::{branch::alt,bytes::complete::tag, sequence::{delimited, preceded}, IResult, character::complete::multispace1, multi::separated_list1, Parser};

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}

impl Monkey {
    fn inspect(&mut self, relief_lowers_worry_level: bool)-> u64 {
       self.touch_count+=1;
        let item = self.items.pop_front().unwrap();
        //println!("");
        let worry_level = match &self.operation {
            Operation::Mul((x, y)) => {
                let num_a = match x {
                    Value::Old=> item,
                    Value::Num(num) =>*num,
                };

                let num_b = match y {
                    Value::Old=> item,
                    Value::Num(num) =>*num,
                };

                num_a * num_b
            }

            Operation::Add((x, y)) => {
                let num_a = match x {
                    Value::Old=> item,
                    Value::Num(num) =>*num,
                };

                let num_b = match y {
                    Value::Old=> item,
                    Value::Num(num) =>*num,
                };

                num_a + num_b
            }
        };

        
        if relief_lowers_worry_level {
            worry_level /3
        }else {
            worry_level
        }
    }

    fn test(&self, item: u64)-> u64 {
        if item % self.test.divisible == 0{
            self.test.true_r
        }else {
            self.test.false_r
        }
    }


}

#[derive(Debug)]
struct Test {
    divisible: u64,
    true_r: u64,
    false_r: u64,
}

#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
//
//
fn operation(input: &str) ->IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;

    let (input, value_2) = value(input)?;

    let result = match operator {
        "*"=> Operation::Mul((value_1, value_2)),
        "+"=> Operation::Add((value_1, value_2)),
        _=>panic!("unknown operator"),
    };
    Ok((input, result))
}
fn value(input: &str) -> IResult<&str, Value> {
    alt((tag("old").map(|_|Value::Old), nom::character::complete::u64.map(Value::Num)))(input)
}

fn test(input: &str)->IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_r) = preceded(tag("If true: throw to monkey "), nom::character::complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_r) = preceded(tag("If false: throw to monkey "), nom::character::complete::u64)(input)?;

    Ok((input, Test{
        divisible,
        true_r,
        false_r,
    }))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(tag("Starting items: "), separated_list1(tag(", "), nom::character::complete::u64))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;
    Ok((input, Monkey{
        items: VecDeque::from(items),
        operation: op,
        test,
        touch_count:0,
    }))
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();

    for _round in 0..20{
      for monkey_index in 0..monkeys.len(){
          // println!("");
          for _ in 0..monkeys[monkey_index].items.len(){
              let monkey =monkeys.get_mut(monkey_index).unwrap();
              let item = monkey.inspect(true);
              let monkey_send_to = monkey.test(item);
              monkeys.get_mut(monkey_send_to as usize).unwrap().items.push_back(item);
          }
      }
    }
    monkeys.sort_by_key(|monkey|monkey.touch_count);
    monkeys.iter().rev().take(2).map(|mk|mk.touch_count).product::<u64>().to_string()
}
pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = include_str!("../test.txt");
    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 10605.to_string());
    }

    #[test]
    #[ignore = "reason"]

    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "".to_string());
    }
}

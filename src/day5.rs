use regex::Regex;

#[derive(Debug)]
struct CrateBox {
    value: String,
}

impl CrateBox {
    fn parse(n: &str) -> Self {
        CrateBox {
            value: n.replace("]", "").replace("[", ""),
        }
    }
    fn copy(&self) -> Self {
        CrateBox {
            value: self.value.to_string(),
        }
    }
}

#[derive(Debug)]
struct PossibleCrates {
    value: Vec<Option<CrateBox>>,
}

impl PossibleCrates {
    fn parse(n: &String) -> Self {
        let mut result = Vec::new();
        let v: Vec<_> = n.split(" ").collect();

        let mut counter = 0;
        for text in n.split(" ") {
            if text.len() == 0 {
                if counter == 3 {
                    result.push(None);
                    counter = 0;
                } else {
                    counter += 1;
                }
            } else {
                result.push(Some(CrateBox::parse(text)));
                counter = 0;
            }
        }
        while result.len() < 9 {
            result.push(None)
        }

        PossibleCrates { value: result }
    }

    fn get_column(&self, index: usize) -> Option<CrateBox> {
        match self.value.get(index) {
            Some(Some(x)) => Some(x.copy()),
            _ => None,
        }
    }

    fn parse_lines(data: &Vec<String>) -> Vec<PossibleCrates> {
        let lines: Vec<_> = data.into_iter().filter(|e| e.contains("[")).collect();
        let mut result = Vec::new();
        for item in lines.into_iter() {
            result.push(PossibleCrates::parse(item));
        }
        result
    }
}

#[derive(Debug)]
struct Stack {
    value: Vec<CrateBox>,
}

impl Stack {
    fn parse(data: &Vec<String>) -> Vec<Stack> {
        let x = PossibleCrates::parse_lines(data);
        let mut result = Vec::new();
        for index in 0..x[0].value.len() {
            let mut inner = Vec::new();
            for item in x.iter() {
                inner.push(item.get_column(index))
            }
            result.push(inner);
        }
        let mut stacks = Vec::new();
        for c in result.into_iter() {
            let mut stack = Vec::new();
            for item in c.into_iter() {
                if let Some(x) = item {
                    stack.push(x);
                }
            }
            stack.reverse();
            stacks.push(Stack { value: stack })
        }
        stacks
    }

    fn take(&mut self, amount: usize) -> Vec<CrateBox> {
        let mut result = Vec::new();
        for _ in 0..amount {
            if let Some(x) = self.value.pop() {
                result.push(x);
            }
        }
        result
    }

    fn push(&mut self, content: Vec<CrateBox>) {
        for n in content.into_iter() {
            self.value.push(n)
        }
    }

    fn print_last(&self) -> String {
        if let Some(x) = self.value.last() {
            format!("{}", x.value)
        } else {
            format!("")
        }
    }
}

#[derive(Debug)]
struct Command {
    from: usize,
    to: usize,
    amount: usize,
}

impl Command {
    fn new(data: &String) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let lst: Vec<usize> = data
            .split(" ")
            .filter(|e| re.is_match(e))
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        Command {
            amount: lst[0],
            from: lst[1] - 1,
            to: lst[2] - 1,
        }
    }
    fn parse(data: &Vec<String>) -> Vec<Command> {
        let text: Vec<_> = data
            .into_iter()
            .filter(|e| e.contains("move"))
            .map(Command::new)
            .collect();
        text
    }
}

fn apply_command(command: &Command, stacks: &mut Vec<Stack>) {
    let mut from = &mut stacks[command.from];
    let content = from.take(command.amount);
    let mut to = &mut stacks[command.to];

    to.push(content);
    // stacks[command.to].push(stacks[command.from].take(command.amount))
}

fn apply_command2(command: &Command, stacks: &mut Vec<Stack>) {
    let mut from = &mut stacks[command.from];
    let mut content = from.take(command.amount);
    let mut to = &mut stacks[command.to];
    content.reverse();
    to.push(content);
    // stacks[command.to].push(stacks[command.from].take(command.amount))
}

fn run_part_1(commands: Vec<Command>, stacks: &mut Vec<Stack>) -> String {
    for command in commands.iter() {
        apply_command(command, stacks);
    }
    stacks.iter().map(|e| e.print_last()).collect()
}

fn run_part_2(commands: Vec<Command>, stacks: &mut Vec<Stack>) -> String {
    for command in commands.iter() {
        apply_command2(command, stacks);
    }
    stacks.iter().map(|e| e.print_last()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn test_stack() {
        let data =
            read_file("/home/wbright/projects/advent_of_code_2022/input/day5.txt".to_string());
        let mut stacks = Stack::parse(&data);
        let command = Command {
            from: 0,
            amount: 1,
            to: 1,
        };
        assert_eq!(8, stacks[0].value.len());
        assert_eq!("S", stacks[0].value.last().unwrap().value);
        assert_eq!(5, stacks[1].value.len());
        apply_command(&command, &mut stacks);
        assert_eq!(7, stacks[0].value.len());
        assert_eq!(6, stacks[1].value.len());
        assert_eq!("S", stacks[1].value.last().unwrap().value);
        assert_eq!("L", stacks[0].value.last().unwrap().value);
    }

    #[test]
    fn test_part1() {
        let data =
            read_file("/home/wbright/projects/advent_of_code_2022/input/day5.txt".to_string());
        let mut stacks = Stack::parse(&data);
        let commands = Command::parse(&data);
        let result = run_part_1(commands, &mut stacks);
        assert_eq!("RNZLFZSJH", result);
    }

    #[test]
    fn test_part2() {
        let data =
            read_file("/home/wbright/projects/advent_of_code_2022/input/day5.txt".to_string());
        let mut stacks = Stack::parse(&data);
        let commands = Command::parse(&data);
        let result = run_part_2(commands, &mut stacks);
        assert_eq!("CNSFCGJSM", result);
    }
}

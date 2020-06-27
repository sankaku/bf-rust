#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pos: usize,
    tape: Vec<u32>,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    IncrPtr,
    DecrPtr,
    IncrVal,
    DecrVal,
    Output,
    // GetChar,
    // IterStart,
    // IterEnd,
}

impl Command {
    pub fn char_to_command(c: char) -> Option<Command> {
        match c {
            '>' => Some(Command::IncrPtr),
            '<' => Some(Command::DecrPtr),
            '+' => Some(Command::IncrVal),
            '-' => Some(Command::DecrVal),
            '.' => Some(Command::Output),
            _ => None,
        }
    }
    fn call(&self, state: &State) -> State {
        let pos = &state.pos;
        let tape = &state.tape;

        match &self {
            Command::IncrPtr => State {
                pos: *pos + 1,
                tape: tape.to_vec(),
            },
            Command::DecrPtr => State {
                pos: *pos - 1,
                tape: tape.to_vec(),
            },
            Command::IncrVal => State {
                pos: *pos,
                tape: {
                    let mut v = tape.to_vec();
                    v[*pos] += 1;
                    v
                },
            },
            Command::DecrVal => State {
                pos: *pos,
                tape: {
                    let mut v = tape.to_vec();
                    v[*pos] -= 1;
                    v
                },
            },
            Command::Output => {
                println!("{:?}", tape[*pos]);
                State {
                    pos: *pos,
                    tape: tape.to_vec(),
                }
            } // Command::GetChar => { },
              // Command::IterStart => (),// TODO
              // Command::IterEnd => (),// TODO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call__must_increment_pos_for_IncrPtr() {
        let command = Command::IncrPtr;
        let state = State {
            pos: 0,
            tape: vec![0],
        };
        let actual = command.call(&state);
        let expected = State {
            pos: 1,
            tape: vec![0],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn call__must_decrement_pos_for_DecrPtr() {
        let command = Command::DecrPtr;
        let state = State {
            pos: 1,
            tape: vec![0],
        };
        let actual = command.call(&state);
        let expected = State {
            pos: 0,
            tape: vec![0],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn call__must_increment_vec_value__for_IncrVal() {
        let command = Command::IncrVal;
        let state = State {
            pos: 0,
            tape: vec![0],
        };
        let actual = command.call(&state);
        let expected = State {
            pos: 0,
            tape: vec![1],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn call__must_decrement_vec_value__for_DecrVal() {
        let command = Command::DecrVal;
        let state = State {
            pos: 0,
            tape: vec![1],
        };
        let actual = command.call(&state);
        let expected = State {
            pos: 0,
            tape: vec![0],
        };
        assert_eq!(actual, expected);
    }
}

pub struct Interpreter {}
impl Interpreter {
    pub fn interpret(s: &str, tape_length: usize) -> Vec<State> {
        let mut states = vec![State {
            pos: 0,
            tape: vec![0; tape_length],
        }];
        let commands = Self::convert_str_to_command(&s);
        for command in commands {
            let latest_state = states
                .last()
                .expect("Can't find the latest state. Something must be wrong!");
            let new_state = command.call(latest_state);
            states.push(new_state);
        }
        states
    }

    fn convert_str_to_command(s: &str) -> Vec<Command> {
        s.chars()
            .fold(Vec::new(), |mut acc, c| match Command::char_to_command(c) {
                Some(command) => {
                    acc.push(command);
                    acc
                }
                None => acc,
            })
    }
}

#[cfg(test)]
mod tests_interpreter {
    use super::*;

    #[test]
    fn interpret_must_work() {
        let s = ">";
        let tape_length = 3;
        let actual = Interpreter::interpret(s, tape_length);
        let expected = vec![
            State {
                pos: 0,
                tape: vec![0; tape_length],
            },
            State {
                pos: 1,
                tape: vec![0; tape_length],
            },
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    fn convert_str_to_command_must_work() {
        let input = ">< and meaningless string";
        let actual = Interpreter::convert_str_to_command(input);
        let expected = vec![Command::IncrPtr, Command::DecrPtr];
        assert_eq!(actual, expected)
    }
}

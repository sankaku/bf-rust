use super::command::Command;
use super::entity::{NextAction, State};

pub struct Interpreter {}
impl Interpreter {
    pub fn interpret(s: &str, tape_length: usize) -> Vec<State> {
        let mut states = vec![State {
            pos: 0,
            tape: vec![0; tape_length],
        }];
        let commands = Self::convert_str_to_command(&s);

        let mut i = 0;
        while i < commands.len() {
            let command = &commands[i];
            let latest_state = states
                .last()
                .expect("Can't find the latest state. Something must be wrong!");
            let (new_state, next_action) = command.call(latest_state);
            match next_action {
                NextAction::GoForward => {
                    i += 1;
                }
                NextAction::JumpBackward => {
                    i = Self::find_corresponding_iter_start(i, &commands) + 1;
                }
                NextAction::JumpForward => {
                    i = Self::find_corresponding_iter_end(i, &commands) + 1;
                }
            }
            // println!("{:?}", new_state);
            states.push(new_state);
        }

        states
    }

    fn convert_str_to_command(s: &str) -> Vec<Command> {
        // TODO use flatten?
        s.chars()
            .fold(Vec::new(), |mut acc, c| match Command::char_to_command(c) {
                Some(command) => {
                    acc.push(command);
                    acc
                }
                None => acc,
            })
    }

    fn find_corresponding_iter_start(i: usize, commands: &Vec<Command>) -> usize {
        let mut j = i - 1;
        let mut num_iter_start = 0;
        let mut num_iter_end = 1;
        while num_iter_start != num_iter_end {
            let tmp_command = &commands[j];
            if tmp_command.is_iter_start() {
                num_iter_start += 1;
            } else if tmp_command.is_iter_end() {
                num_iter_end += 1;
            }
            j -= 1;
        }
        j
    }

    fn find_corresponding_iter_end(i: usize, commands: &Vec<Command>) -> usize {
        let mut j = i + 1;
        let mut num_iter_start = 1;
        let mut num_iter_end = 0;
        while num_iter_start != num_iter_end {
            let tmp_command = &commands[j];
            if tmp_command.is_iter_start() {
                num_iter_start += 1;
            } else if tmp_command.is_iter_end() {
                num_iter_end += 1;
            }
            j += 1;
        }
        j
    }
}

#[cfg(test)]
mod tests {
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
    fn interpret_must_work_for_loop() {
        let s = "++[-]";
        let tape_length = 1;
        let actual = Interpreter::interpret(s, tape_length);
        let expected = vec![
            State {
                pos: 0,
                tape: vec![0],
            },
            State {
                pos: 0,
                tape: vec![1],
            },
            State {
                pos: 0,
                tape: vec![2],
            },
            State {
                pos: 0,
                tape: vec![2],
            },
            State {
                pos: 0,
                tape: vec![1],
            },
            State {
                pos: 0,
                tape: vec![1],
            },
            State {
                pos: 0,
                tape: vec![1],
            },
            State {
                pos: 0,
                tape: vec![0],
            },
            State {
                pos: 0,
                tape: vec![0],
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

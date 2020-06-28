use super::entity::{NextAction, State};
use std::char;
use std::io::{stdin, Read};
use std::io::{stdout, Write}; // To flush in GetChar

#[derive(Debug, PartialEq)]
pub enum Command {
    IncrPtr,
    DecrPtr,
    IncrVal,
    DecrVal,
    Output,
    GetChar,
    IterStart,
    IterEnd,
}

impl Command {
    pub fn char_to_command(c: char) -> Option<Command> {
        match c {
            '>' => Some(Command::IncrPtr),
            '<' => Some(Command::DecrPtr),
            '+' => Some(Command::IncrVal),
            '-' => Some(Command::DecrVal),
            '.' => Some(Command::Output),
            ',' => Some(Command::GetChar),
            '[' => Some(Command::IterStart),
            ']' => Some(Command::IterEnd),
            _ => None,
        }
    }

    pub fn is_iter_start(&self) -> bool {
        *self == Command::IterStart
    }

    pub fn is_iter_end(&self) -> bool {
        *self == Command::IterEnd
    }

    pub fn call(&self, state: &State) -> (State, NextAction) {
        let pos = &state.pos;
        let tape = &state.tape;

        // TODO return just `state` when input equals output
        match &self {
            Command::IncrPtr => (
                State {
                    pos: *pos + 1,
                    tape: tape.to_vec(),
                },
                NextAction::GoForward,
            ),
            Command::DecrPtr => (
                State {
                    pos: *pos - 1,
                    tape: tape.to_vec(),
                },
                NextAction::GoForward,
            ),
            Command::IncrVal => (
                State {
                    pos: *pos,
                    tape: {
                        let mut v = tape.to_vec();
                        v[*pos] += 1;
                        v
                    },
                },
                NextAction::GoForward,
            ),
            Command::DecrVal => (
                State {
                    pos: *pos,
                    tape: {
                        let mut v = tape.to_vec();
                        v[*pos] -= 1;
                        v
                    },
                },
                NextAction::GoForward,
            ),
            Command::Output => {
                print!("{}", tape[*pos] as char);
                (
                    State {
                        pos: *pos,
                        tape: tape.to_vec(),
                    },
                    NextAction::GoForward,
                )
            }
            Command::GetChar => {
                print!("INPUT: ");
                stdout().flush().unwrap();
                let mut buf: Vec<u8> = vec![0];
                stdin().lock().read(&mut buf).expect("Input error");
                (
                    State {
                        pos: *pos,
                        tape: {
                            let mut v = tape.to_vec();
                            v[*pos] = *buf.first().unwrap();
                            v
                        },
                    },
                    NextAction::GoForward,
                )
            }
            Command::IterStart => {
                let next_action = if tape[*pos] == 0 {
                    NextAction::JumpForward
                } else {
                    NextAction::GoForward
                };
                (
                    State {
                        pos: *pos,
                        tape: tape.to_vec(),
                    },
                    next_action,
                )
            }
            Command::IterEnd => {
                let next_action = if tape[*pos] == 0 {
                    NextAction::GoForward
                } else {
                    NextAction::JumpBackward
                };
                (
                    State {
                        pos: *pos,
                        tape: tape.to_vec(),
                    },
                    next_action,
                )
            }
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
        let expected = (
            State {
                pos: 1,
                tape: vec![0],
            },
            NextAction::GoForward,
        );
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
        let expected = (
            State {
                pos: 0,
                tape: vec![0],
            },
            NextAction::GoForward,
        );
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
        let expected = (
            State {
                pos: 0,
                tape: vec![1],
            },
            NextAction::GoForward,
        );
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
        let expected = (
            State {
                pos: 0,
                tape: vec![0],
            },
            NextAction::GoForward,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn is_iter_start_must_work_for_iter_start() {
        let command = Command::IterStart;
        let actual = command.is_iter_start();
        let expected = true;
        assert_eq!(actual, expected)
    }

    #[test]
    fn is_iter_start_must_work_for_iter_end() {
        let command = Command::IterEnd;
        let actual = command.is_iter_start();
        let expected = false;
        assert_eq!(actual, expected)
    }

    #[test]
    fn is_iter_end_must_work_for_iter_end() {
        let command = Command::IterEnd;
        let actual = command.is_iter_end();
        let expected = true;
        assert_eq!(actual, expected)
    }
}

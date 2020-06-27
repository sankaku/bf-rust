#[derive(Debug, PartialEq)]
pub struct State {
    pos: usize,
    tape: Vec<u32>,
}

pub enum Command {
    IncrPtr,
    DecrPtr,
    IncrVal,
    DecrVal,
    // Output,
    // GetChar,
    // IterStart,
    // IterEnd,
}

impl Command {
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
            // Output => println!("{}", tape[pos]),
            // GetChar => (),// TODO
            // IterStart => (),// TODO
            // IterEnd => (),// TODO
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

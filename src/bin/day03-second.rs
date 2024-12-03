use std::cmp::PartialEq;
use std::io::Read;

#[derive(Eq, PartialEq)]
enum State {
    Invalid,
    M,
    U,
    L,
    LeftParenthesis,
    FirstNumber1,
    FirstNumber2,
    FirstNumber3,
    Comma,
    SecondNumber1,
    SecondNumber2,
    SecondNumber3,
    RightParenthesis,
}

#[derive(Eq, PartialEq)]
enum DoState {
    Invalid,
    D,
    O,
    N,
    Apostrophe,
    T,
    DoLeftParenthesis,
    DoRightParenthesis,
    DontLeftParenthesis,
    DontRightParenthesis,
}

fn main() -> Result<(), ()> {
    let mut args = std::env::args();

    let s = match args.len() {
        2 => {
            //read file
            let filename = args.nth(1).unwrap();
            std::fs::read_to_string(filename)
        }
        _ => {
            //read stdin
            let mut buffer = String::new();
            match std::io::stdin().read_to_string(&mut buffer) {
                Ok(_) => Ok(buffer),

                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "could not read stdin",
                )),
            }
        }
    };

    match s {
        Ok(s) => {
            let line = s.trim();

            let mut total = 0;

            let mut state = State::Invalid;
            let mut first_num = 1;
            let mut second_num = 1;

            let mut do_state = DoState::Invalid;
            let mut enabled = true;

            for c in line.chars() {
                match c {
                    'm' => {
                        state = State::M;
                    }
                    'u' => {
                        if state == State::M {
                            state = State::U;
                        } else {
                            state = State::Invalid;
                        }
                    }

                    'l' => {
                        if state == State::U {
                            state = State::L;
                        } else {
                            state = State::Invalid;
                        }
                    }

                    '(' => {
                        if state == State::L {
                            state = State::LeftParenthesis;
                        } else {
                            state = State::Invalid;
                        }
                    }

                    '0'..='9' => {
                        let num = c.to_digit(10).unwrap();

                        match state {
                            State::LeftParenthesis => {
                                first_num = num;
                                state = State::FirstNumber1;
                            }
                            State::FirstNumber1 => {
                                first_num *= 10;
                                first_num += num;
                                state = State::FirstNumber2;
                            }

                            State::FirstNumber2 => {
                                first_num *= 10;
                                first_num += num;
                                state = State::FirstNumber3;
                            }

                            State::Comma => {
                                second_num = num;
                                state = State::SecondNumber1;
                            }

                            State::SecondNumber1 => {
                                second_num *= 10;
                                second_num += num;
                                state = State::SecondNumber2;
                            }
                            State::SecondNumber2 => {
                                second_num *= 10;
                                second_num += num;
                                state = State::SecondNumber3;
                            }

                            _ => {
                                state = State::Invalid;
                            }
                        }
                    }

                    ',' => {
                        if state == State::FirstNumber1
                            || state == State::FirstNumber2
                            || state == State::FirstNumber3
                        {
                            state = State::Comma;
                        } else {
                            state = State::Invalid;
                        }
                    }

                    ')' => {
                        if state == State::SecondNumber1
                            || state == State::SecondNumber2
                            || state == State::SecondNumber3
                        {
                            state = State::RightParenthesis;
                            // println!("first: {}, second: {}", first_num, secondNum);
                            if enabled {
                                total += first_num * second_num;
                            }
                        } else {
                            state = State::Invalid;
                        }
                    }

                    _ => {
                        state = State::Invalid;
                    }
                }

                match c {
                    'd' => {
                        do_state = DoState::D;
                    }

                    'o' => {
                        if do_state == DoState::D {
                            do_state = DoState::O;
                        } else {
                            do_state = DoState::Invalid;
                        }
                    }

                    'n' => {
                        if do_state == DoState::O {
                            do_state = DoState::N;
                        }
                        else {
                            do_state = DoState::Invalid;
                        }
                    }

                    '\'' => {
                        if do_state == DoState::N {
                            do_state = DoState::Apostrophe;
                        }
                        else {
                            do_state = DoState::Invalid;
                        }
                    }

                    't' => {
                        if do_state == DoState::Apostrophe {
                            do_state = DoState::T;
                        }
                        else {
                            do_state = DoState::Invalid;
                        }
                    }

                    '(' => {
                        if do_state == DoState::O {
                            do_state = DoState::DoLeftParenthesis;
                        } else if do_state == DoState::T {
                            do_state = DoState::DontLeftParenthesis;
                        } else {
                            do_state = DoState::Invalid;
                        }
                    }

                    ')' => {
                        if do_state == DoState::DoLeftParenthesis {
                            do_state = DoState::DoRightParenthesis;
                            enabled = true;
                        } else if do_state == DoState::DontLeftParenthesis {
                            do_state = DoState::DontRightParenthesis;
                            enabled = false;
                        } else {
                            do_state = DoState::Invalid;
                        }
                    }
                    _ => {
                        do_state = DoState::Invalid;
                    }
                }
            }

            println!("{}", total);

            Ok(())
        }

        Err(_) => Err(()),
    }
}

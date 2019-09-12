use std::io;

const STACK_LEN : usize = 4096;

fn main() {
    let mut code : String = String::new();
    io::stdin().read_line(&mut code).unwrap();

    interpeter(&code);
}

fn interpeter(code : &String) {

    let code_array = code.as_bytes();
    let end = code_array.len();

    let mut cp: usize = 0;
    let mut sp: usize = 0;
    let mut jsp: usize = 0;
    let mut stack: [u8; STACK_LEN] = [0; STACK_LEN];
    let mut jump_stack: [usize; STACK_LEN] = [0; STACK_LEN];

    let mut stdin_buffer : String = String::new();
    let mut stdin_current = 0;

    while cp < end {
        match code_array[cp] {
            b'>' => sp += 1,
            b'<' => sp -= 1,
            b'+' => stack[sp] += 1,
            b'-' => stack[sp] -= 1,
            b'.' => print!("{}", stack[sp] as char),
            b',' => {
                if stdin_current == stdin_buffer.len() {
                    io::stdin().read_line(&mut stdin_buffer).unwrap();
                    stdin_current = 0;
                }
                stack[sp] = stdin_buffer.as_bytes()[stdin_current];
                stdin_current += 1;
            },
            b'[' => {
                if stack[sp] == 0 {
                    cp += 1;
                    let mut need_paried_bracket : usize = 0;
                    let mut found : bool = false;
                    while cp < end {
                        if code_array[cp] == b'[' {
                            need_paried_bracket += 1;
                        } else if code_array[cp] == b']' {
                            if need_paried_bracket == 0 {
                                found = true;
                            } else {
                                need_paried_bracket -= 1;
                            }
                        }
                        if found {
                            break;
                        }
                        cp += 1;
                    }
                } else {
                    jump_stack[jsp] = cp;
                    jsp += 1;
                }
            },
            b']' => {
                if stack[sp] != 0 {
                    cp = jump_stack[jsp - 1];
                } else {
                    jsp -= 1;
                }
            },
            b'\n' => {
                // Just ignore
            },
            _ => panic!(),
        };
        cp += 1;
    }
}

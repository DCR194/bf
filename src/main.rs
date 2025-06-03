use std::{
    char, env, fs,
    io::{self, Read},
};

struct Program {
    instructions: String,
    pc: u32, // Program Counter
    tape: Vec<u8>,
    dp: u32, // Data Pointer
}

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("error: {error}"),
    }
    input
}

fn increment_pc(program: &mut Program) {
    program.pc += 1;
}

fn handle_increment_dp(program: &mut Program) {
    program.dp += 1;
    if program.tape.len() < program.dp as usize {
        program.tape.resize(program.dp as usize * 2, 0);
    }
}

fn handle_decrement_dp(program: &mut Program) {
    program.dp -= 1;
}

fn handle_increment_databyte(program: &mut Program) {
    program.tape[program.dp as usize] += 1;
}

fn handle_decrement_databyte(program: &mut Program) {
    program.tape[program.dp as usize] -= 1;
}

fn handle_print_char(program: &mut Program) {
    print!("{}", program.tape[program.dp as usize] as char);
}

fn handle_accept_char(program: &mut Program) -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut buffer = [0_u8; 1];

    // Read a single byte
    stdin.read_exact(&mut buffer)?;

    // Extract the byte value
    let byte = buffer[0];

    // Check if the byte is within the ASCII range (0-127)
    if byte < 128 {
        // If it's ASCII, convert it to a char
        // let char_value = byte as char;
        // println!("Read character: {}", char_value);

        program.tape[program.dp as usize] = byte;
    } else {
        panic!("Read non-ASCII character. More complex decoding needed.");
    }

    Ok(())
}

// Handle `[` instruction
// Find correct closing `]`
// If the byte at the pointer is 0 skip ahead after `]`
fn handle_open_bracket(program: &mut Program) {
    if program.tape[program.dp as usize] != 0 {
        return;
    }

    let mut count = 1;

    // Increment Program Counter by 1
    program.pc += 1;

    //Increment Program Counter until we find the matching ]
    //This is a naive interpreter for brainf*ck so we don't care too much for wrong input
    //If you're asking why I am not explicitly checking for errors: **** you, that's why
    while count > 0 {
        match program.instructions.as_bytes()[program.pc as usize] as char {
            '[' => count += 1,
            ']' => count -= 1,
            _ => {}
        }
        // Yes, I could've just written += 1 but I like this better
        increment_pc(program);
    }
}

//If the byte at the data pointer is not zero, jump the data pointer
// Until after the matching initial `[`
fn handle_close_bracket(program: &mut Program) {
    if program.tape[program.dp as usize] == 0 {
        return;
    }

    let mut count = 1;

    // Now we decrement because we want to go back
    program.pc -= 1;

    while count > 0 {
        match program.instructions.as_bytes()[program.pc as usize] as char {
            '[' => count -= 1,
            ']' => count += 1,
            _ => {}
        }
        // Okay sure, say what you want about it being different from the other one
        // I AM NOT GOING TO PRETEND THIS WAS A VERY WELL THOUGHT OUT PROJECT
        // IT IS 1 AM RIGHT BEFORE A FLIGHT, cut me some slack
        program.pc -= 1;
    }
}

fn run_bf(input: String) {
    let mut program = Program {
        instructions: input,
        pc: 0,
        tape: Vec::new(),
        dp: 0,
    };
    program.tape.resize(10000, 0);
    while (program.pc as usize) < program.instructions.len() {
        // print!("DEBUG");
        match program.instructions.as_bytes()[program.pc as usize] as char {
            '>' => handle_increment_dp(&mut program),
            '<' => handle_decrement_dp(&mut program),
            '+' => handle_increment_databyte(&mut program),
            '-' => handle_decrement_databyte(&mut program),
            '.' => handle_print_char(&mut program),
            ',' => handle_accept_char(&mut program).expect("Wrong Input"),
            '[' => handle_open_bracket(&mut program),
            ']' => handle_close_bracket(&mut program),
            _ => {}
        }
        increment_pc(&mut program);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let manual_input = get_input();
        run_bf(manual_input);
        return;
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap();
    run_bf(contents);
}

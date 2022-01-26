use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const RTYPE_INST: [&str; 5] = ["understeer", "oversteer", "overtake", "sidebysideintot1","pitstop"];
const JTYPE_INST: [&str; 1] = ["divebomb"];
const STYPE_INST: [&str; 2] = ["radiomessage","steering"];

fn main() {
    let mut registers: [i32; 4] = [0; 4];
    

    let lines: Vec<String> = read_lines(Path::new("./fac.formula1"));

    let mut line: usize = 0;
    while line < lines.len() {
        check_syntax(&lines[line], line);
        let curr_inst: Vec<&str> = lines[line].split_whitespace().collect();
        let inst = curr_inst[0];

        if inst == "steering" {
            registers[1] = read_input();
        }
        else if inst == "radiomessage" {
            println!("{}", registers[1]);
        }
        else if inst == "oversteer" {
            let args = parse_r_type(curr_inst);
            registers[args[0]] += match args[2] {
                1 => (args[2] as i32),
                _ => registers[args[1]]
            }
        }
        else if inst == "understeer" {
            let args = parse_r_type(curr_inst);
            registers[args[0]] -= match args[2] {
                1 => (args[2] as i32),
                _ => registers[args[1]]
            }
        }
        else if inst == "overtake" {
            let args = parse_r_type(curr_inst);
            registers[args[0]] = registers[args[1]] - (args[2] as i32);
        }
        else if inst == "pitstop" {
            let args = parse_r_type(curr_inst);
            registers[args[0]] = args[2] as i32;
        }
        else if inst == "divebomb" {
            let mv = curr_inst[1].parse::<isize>().unwrap();
            match mv {
                0.. => line += mv as usize,
                _ => line -= (-mv) as usize
            }
        }
        else if inst == "sidebysideintot1" {
            let args = parse_r_type(curr_inst);
            if registers[args[0]] == args[2] as i32 {
                line += 1;
            }
        }

        line += 1;
    }
}

fn read_input() -> i32 {
    let input = std::io::stdin();

    let mut lines = input
         .lock()
         .lines()
         .map(|_line| _line.ok().unwrap());

    return lines.next().unwrap().parse::<i32>().unwrap()
}

fn parse_r_type(_inst: Vec<&str>) -> [usize; 3] {
    let mut ret: [usize; 3] = [0; 3];
    ret[0] = _inst[1].strip_prefix("P").unwrap().parse::<usize>().unwrap();
    ret[1] = _inst[2].strip_prefix("P").unwrap().parse::<usize>().unwrap();
    ret[2] = _inst[3].parse::<usize>().unwrap();
    ret
}

fn check_syntax(_code: &String, _line: usize) {
    let mut errors: usize = 0;
    let elements: Vec<&str> = _code.split_whitespace().collect();
    // check if r-type instruction
    if RTYPE_INST.iter().any(|i| *i == elements[0]) {
        // check if correct length of instruction
        if elements.len() == 4 {
            // check if registers are correct
            for i in [1 as usize,2 as usize] {
                let cs: Vec<char> = elements[i].chars().collect();
                if cs[0 as usize] != 'P' {
                    println!("\tError on line {}: Missing P to mark registry", _line);
                    errors += 1;
                }
                if !['0','1','2','3'].iter().any(|c| *c == cs[1 as usize]) {
                    println!("\tError on line {}: There are only 4 registers, P0, P1, P2 and P3", _line);
                    errors += 1;
                }
            }
            // check if immidiate is within specification
            if elements[3] != "0" && elements[3] != "1" {
                println!("\tError on line {}: invalid immidiate. Can only be a 1 bit value", _line);
            }
        }
        else {
            println!("\tError on line {}: Incorrect instruction of length {}", _line, elements.len());
            errors += 1;
        }
    }
    // check if j-type instruction
    else if JTYPE_INST.iter().any(|i| *i == elements[0]) {
        // check if correct length of instruction
        if elements.len() != 2 {
            println!("\tError on line {}: Incorrect instruction of length {}", _line, elements.len());
            errors += 1;
        }
        // check if immidiate is within specification
        if elements[1].parse::<i8>().unwrap() > 15 || elements[1].parse::<i8>().unwrap() < -16 {
            println!("\tError on line {}: {} does not fit in a 5 bit signed value", _line, elements[1].parse::<isize>().unwrap());
            errors += 1;
        }
    }
    // check if s-type instruction
    else if STYPE_INST.iter().any(|i| *i == elements[0]) {
        // check if correct length of instruction
        if elements.len() != 1 {
            println!("\tError on line {}: Incorrect instruction of length {}", _line, elements.len());
            errors += 1;
        }
    }
    else {
        println!("\tError on line {}: {} is not an existing instruction in this language", _line, elements[0]);
        errors += 1; 
    }

    
    if errors > 0 {
        println!("\tCode has {} syntax errors. Unable to run.", errors);
        std::process::exit(1);
    }
}


fn read_lines(_p: &Path) -> Vec<String> {
    let lines: Vec<String>;

    match File::open(_p) {
        Ok(f) => {
            lines = io::BufReader::new(f).lines().map(|l| l.ok().unwrap()).collect();
        
        },
        _ => {
            println!("Unable to read file");
            std::process::exit(1);
        }
    };

    lines
}


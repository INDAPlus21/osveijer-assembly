use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut registers: [i32; 4] = [0; 4];
    let rtype_inst: [&str; 5] = ["understeer", "oversteer", "overtake", "sidebysideintot1","pitstop"];
    let jtype_inst: [&str; 5] = ["divebomb"; 5];
    let stype_inst: [&str; 5] = ["radiomessage","radiomessage","steering","steering","steering"];

    let insts = [rtype_inst, jtype_inst, stype_inst];

    let path = Path::new("./fac.formula1");
    let lines = read_lines(path);

    check_syntax(&lines, insts);

    let mut line: usize = 0;
    while line < lines.len() {
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

fn check_syntax(_code: &Vec<String>, _insts: [[&str; 5]; 3]) {
    let mut errors: usize = 0;
    for line in _code {
        let elements: Vec<&str> = line.split_whitespace().collect();
        if _insts[0].iter().any(|i| *i == elements[0]) {
            if elements.len() == 4 {
                for i in [1 as usize,2 as usize] {
                    let cs: Vec<char> = elements[i].chars().collect();
                    if cs[0 as usize] != 'P' {
                        println!("\tError on line {}: Missing P to mark registry", _code.iter().position(|l| l == line).unwrap());
                        errors += 1;
                    }
                    if !['0','1','2','3'].iter().any(|c| *c == cs[1 as usize]) {
                        println!("\tError on line {}: There are only 4 registers, P0, P1, P2 and P3", _code.iter().position(|l| l == line).unwrap());
                        errors += 1;
                    }
                }
                if elements[3] != "0" && elements[3] != "1" {
                    println!("\tError on line {}: invalid immidiate. Can only be a 1 bit value", _code.iter().position(|l| l == line).unwrap());
                }
            }
            else {
                println!("\tError on line {}: Incorrect instruction of length {}", _code.iter().position(|l| l == line).unwrap(), elements.len());
                errors += 1;
            }
        }
        else if _insts[1].iter().any(|i| *i == elements[0]) {
            if elements.len() != 2 {
                println!("\tError on line {}: Incorrect instruction of length {}", _code.iter().position(|l| l == line).unwrap(), elements.len());
                errors += 1;
            }
            if elements[1].parse::<i8>().unwrap() > 15 || elements[1].parse::<i8>().unwrap() < -16 {
                println!("\tError on line {}: {} does not fit in a 5 bit signed value", _code.iter().position(|l| l == line).unwrap(), elements[1].parse::<i8>().unwrap());
                errors += 1;
            }
        }
        else if _insts[2].iter().any(|i| *i == elements[0]) {
            if elements.len() != 1 {
                println!("\tError on line {}: Incorrect instruction of length {}", _code.iter().position(|l| l == line).unwrap(), elements.len());
                errors += 1;
            }
        }
        else {
            println!("\tError on line {}: {} is not an existing instruction in this language", _code.iter().position(|l| l == line).unwrap(), elements[0]);
            errors += 1; 
        }
    }
    
    if errors > 0 {
        println!("\tCode has {} syntax errors. Unable to run.", errors);
        std::process::exit(1);
    }
}

fn read_lines(_p: &Path) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    match File::open(_p) {
        Ok(f) => {
            lines = io::BufReader::new(f).lines().map(|l| l.ok().unwrap()).collect()
        
        },
        _ => {}
    };

    lines
}

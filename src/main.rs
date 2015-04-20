//#![feature(core)]

use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::fs::File;
use std::io::Result;

const PLUS : u8 = '+' as u8;
const MINUS : u8 = '-' as u8;
const INC_DATA : u8 = '>' as u8;
const DEC_DATA : u8 = '<' as u8;
const OUTPUT : u8 = '.' as u8;
const INPUT : u8 = ',' as u8;
const OPEN : u8 = '[' as u8;
const CLOSE : u8 = ']' as u8;


fn get_progn(filename: &str) -> Result<Vec<u8>> {
    let mut f = try!(File::open(filename));
    
    let mut buf_progn = Vec::new();

    try!(f.read_to_end(&mut buf_progn));

    Ok(buf_progn)
}


fn main() {
    // TODO read the source file from the args
    
    // This could be, in theory, a Vec, but I don't want to do the logic to autogrow it. 
    // For now it's just easy to allocate X cells.  
    // Here we are using 100 of i8s, but the wikipedia entry says 30,000 is typical. 
    // I tried u8 but one the rot13 gets underflow math errors.
    let mut data : [i8 ; 100] = [0; 100];
    
    let progn = match get_progn("source2.txt") {
        Ok(s) => s,
        Err(e) => {
            println!("Couldn't acquire program source code 'source.txt': {}", e);
            return;
        }
    };

    let mut ptr : usize = 0;
    let mut pc : usize = 0;
    let mut stdin = stdin();
    let mut stdout = stdout();

    let mut branches : Vec<usize> = Vec::new();
    
    let progn_size = progn.len();
    //println!("Program size: {}, PC={}", progn_size, pc);

    while pc < progn_size {
        //println!("PC {}, cmd: {}", pc, progn[pc]);
        match progn[pc] {
            PLUS => {
                data[ptr] += 1;
            },
            MINUS => {
                data[ptr] -= 1;
            },
            INC_DATA => {
                ptr += 1;
            },
            DEC_DATA => {
                if ptr == 0 {
                    // debug tracing
                    print_snippet(&progn, pc);
                    dump_cells(&data);
                    println!("data pointer already at 0 at pc {}", pc);
                    panic!("data pointer underflow");
                }
                ptr -= 1;
            },
            OUTPUT => {
                let outbuf : [u8; 1] = [data[ptr] as u8];
                stdout.write(&outbuf).unwrap();
            },
            INPUT => {
                let mut inp : [u8 ; 1] = [0];
                let len = stdin.read(&mut inp).unwrap();
                if len == 0 {
                    data[ptr] = -1;
                } else {
                    data[ptr] = inp[0] as i8;
                }
            },
            OPEN => {
                if data[ptr] == 0 {
                    seek_to_next_balanced_close(&progn, &mut pc);
                    // the PC will be pointing to ] and the pc+=1 will be the right thing
                } else {
                    // continue forward....
                    branches.push(pc);
                }
            },
            CLOSE => {
                if data[ptr] == 0 {
                    branches.pop();
                } else {
                    pc = *branches.last().unwrap();
                }
            },
            _ => {
                // ignore
            }
        }
 
        pc += 1;
    }
}

// fn print_type_of<T>(_: &T) {
//      let type_name = 
//         unsafe {
//             std::intrinsics::type_name::<T>()
//         };
//     println!("{}", type_name);
// }

// push the referenced program counter forward until the next
// matching close bracket ].  Leave the PC pointing at the ]. 
fn seek_to_next_balanced_close(progn : &Vec<u8>, pc : &mut usize) {
    let mut opens = 0;
    *pc += 1;
    loop {
        match progn[*pc] {
            OPEN => {
                //println!("open at {}", *pc);
                opens += 1;
            },
            CLOSE => {
                if opens == 0 {
                    //println!("final balanced close at {}", *pc);
                    return;
                } else {
                    //println!("close to balance an open at {}", *pc);
                    opens -= 1;
                }
            },
            _ => { }
        }
        *pc += 1;
    }
}

fn dump_cells(data : &[i8]) {
    for n in data {
        print!("{} ", n) ;
    }
    println!("");
}

fn print_snippet(progn : &Vec<u8> , pc : usize) {
    let siz : i32 = pc as i32;
    let start = max(0, siz-3);
    
    let end = min(siz+3, progn.len() as i32 - 1 );

    // turn the u8 -> string.
    let mut str = String::new();
    for n in start..end {
        str.push(progn[n as usize] as char);
    }
    println!("code: {}", str);
}

fn max<T : Ord >( a : T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

fn min<T : Ord>(a : T, b : T) -> T {
    if a < b {
        a 
    } else {
        b
    }
}

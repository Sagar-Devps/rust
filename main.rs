 
//  fn main()
// {
//    let  age=40;
//    let  name="jack";
//     println!("Age={}",age);
//     println!("name={}",name );
// }
// fn main(){
//   let integar:u8=89;
//   let char=integar as char;
//   println!("{}",char);
//   let a=5.6;
//   let b=4.5;
//   let y=a/b;
//   println!("{}",y);

// }
// fn main(){
//     let a=7;
//     let condition = a<5;
//     println!("{}",condition);


//to use recursion
// fn main(){
//     let x=12;
//     let  mut w=1;
//     while  w<=10
//     {
//         let m=w*x;
//         println!("{}*{}={}",x,w,m);
//         w +=1;
//     }
// }
// fn main(){
//     let mut a=3;
//     loop {
//         a+=1;
//         if a>=50{
//             break;
//         }
//         println!("{}",a);
//     }
// }
// for printing the star pattern
// fn main()
//  {
//     let mut x=4;
//     while x<=10
//     {
//         let mut y=1;
//         while y<=x
//         {
//             print!("*");
//             y+=1;
//         }
//         println!();
//         x+=1;
//     }
//  }
// fn main(){
//     let numbers=[1,2,3,4,5,6];
//     println!("the array of numbers are={:?}",numbers);
// }

//to find the greatest number
// use std::io;
// fn main()
// {
//     let mut a=String::new();
//     println!("enter the first number");
//     io::stdin().read_line(&mut a).expect("failed to read line");
//     let a:i128=a.trim().parse().expect("please enter the number");
//     let mut b=String::new();
//     println!("enter the second number");
//     io::stdin().read_line(&mut b).expect("failed to read line");
//     let b:i128=b.trim().parse().expect("please enter the number");
//     let mut c=String::new();
//     println!("enter the third number");
//     io::stdin().read_line(&mut c).expect("failed to read line");
//     let c:i128=c.trim().parse().expect("please enter the number");
//     if a>b && a>c
//     {
//         println!("the greatest number is={}",a);
//     }
//     else if b>a && b>c
//     {
//         println!("the greatest number is={}",b);
//     }
//     else
//     {
//         println!("the greatest number is={}",c);
//     }
// }
//slicing and concating
// fn main()
// {
//     let a="ghjhhjgj";
//     let show=&a[0..4];
//     println!("{}",show);
// }
// fn main(){
//     let a="sagar".to_string();
//     let b="ssagar".to_string();
//     let c=a+&b;
//     println!("{}",c);
// }

// mod ss;

// fn main(){
//     ss::printme();
//     ss::printu();
    
// }  

/*
   simple rust calculator
   add_op := '+' | '-'
   mul_op := '*' | '/'
   digits := {'+' | '-'} [0..9] {[0..9]}
   
   expr := term {add_op term}
   term := factor {mul_op factor}
   factor := digits | '(' expr ')'
   
 */

use std::io;

type CalcInt = i64;

struct ParseState {
    line: String,
    index: usize,
}

fn token(ps: &ParseState) -> char {
    ps.line.as_bytes()[ps.index-1] as char
}

fn fatal(msg: String) {
    eprintln!("fatal: {} ", msg);
    ::std::process::exit(1);
}

fn lex_match(ps: &mut ParseState, expected: char) {
    if token(ps) == expected {
        ps.index += 1;
        return
    }

    fatal(format!("error matching {} at index {}", expected, ps.index));
    std::process::exit(1);
}

fn scan_digits(ps: &mut ParseState) -> CalcInt {
    const BASE: CalcInt = 10;

    let mut val: CalcInt = 0;
    loop {
        let digit: CalcInt;
        match token(ps) {
            '0' => digit = 0,
            '1' => digit = 1,
            '2' => digit = 2,
            '3' => digit = 3,
            '4' => digit = 4,
            '5' => digit = 5,
            '6' => digit = 6,
            '7' => digit = 7,
            '8' => digit = 8,
            '9' => digit = 9,
            _ => break
        }

        if digit >= BASE {
            fatal(format!("Digit {} out of range for base {}", digit, BASE));
        }

        if val > (std::i64::MAX - digit)/BASE {
            fatal(format!("Integer overflow"));
        }

        val = val*BASE + digit;
        ps.index += 1;
    }

    
    return val;
}

fn factor(ps: &mut ParseState) -> CalcInt {
    let value: CalcInt;

    if token(ps) == '(' {
        lex_match(ps, '(');
        value = expr(ps);
        lex_match(ps, ')');
        
    } else if token(ps).is_digit(10) || token(ps) == '+' || token(ps) == '-' {
        
        value = scan_digits(ps);
        
    } else {
        value = 0;
        fatal("bad factor".to_string());
    }

    return value;
}

fn term(ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = factor(ps);

    while token(ps) == '*' || token(ps) == '/' {
        match token(ps) {
            '*' => {
                lex_match(ps, '*');

                value *= factor(ps);
            },

            '/' => {
                lex_match(ps, '/');

                value /= factor(ps);
            },

            _ => {},
        }
    }

    return value;
}

fn expr(ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = term(ps);
    
    match token(ps) {
        '+' => {
            lex_match(ps, '+');
            value += term(ps);
        }
        '-' => {
            lex_match(ps, '-');
            value -= term(ps);
        }
        _ => {},
    }
    
    return value;
}

fn main() {

    let mut ps = ParseState{
        line: String::new(),
        index: 1, // index of NEXT char to read
    };

    print!("expr:\n\t");

    match io::stdin().read_line(&mut ps.line) {
        Ok(_n) => {
            
            let result = expr(&mut ps);
            
            println!("result: {}", result);
        }
        
        Err(error) => fatal(format!("error: {}", error)),
    }
    
}
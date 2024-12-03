use std::io::BufRead;
use regex::Regex;

macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        {
            println!($($arg)*);
        }
    }
}


fn main() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        // Read whole input from stdin
        for line in lock.lines() {
            input.push_str(&line.unwrap());
        }
    }
    dprintln!("input: {:?}", input);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: i32 = re.captures_iter(&input)
        .filter(|c| {
            let capture = c.get(0).unwrap();
            let do_index = input[..capture.start()].rfind("do()").unwrap_or(usize::MAX);
            let dont_index = input[..capture.start()].rfind("don't()").unwrap_or(usize::MAX);
            
            dprintln!("do_index: {}, dont_index: {}", do_index, dont_index);
            match (dont_index == usize::MAX, do_index == usize::MAX) {
                (true, _) => true,
                (false, true) => false,
                (false, false) if do_index > dont_index => true,
                (false, false) => false,
            }
        })
        .map(|c| {
            dprintln!("Capture: {:?}", c);
            let a = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            return [a, b]
        })
        .map(|[a, b]| a * b)
        .sum();

    println!("Result: {}", result);

}

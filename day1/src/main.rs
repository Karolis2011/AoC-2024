use std::io::BufRead;

macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        {
            println!($($arg)*);
        }
    }
}

fn main() {
    let mut list_a = vec![];
    let mut list_b = vec![];

    // Read input from stdin
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();
        // Read input line by line
        for line in lock.lines() {
            let line = line.unwrap();
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| i32::from_str_radix(s, 10).unwrap())
                .take(2)
                .for_each(|x| {
                    if list_a.len() <= list_b.len() {
                        list_a.push(x);
                    } else {
                        list_b.push(x);
                    }
                });
        }
    }

    dprintln!("List A: {:?}", list_a);
    dprintln!("List B: {:?}", list_b);

    #[cfg(feature = "part1")]
    {
        // Sort the lists
        list_a.sort();
        list_b.sort();

        dprintln!("Sorted List A: {:?}", list_a);
        dprintln!("Sorted List B: {:?}", list_b);

        let distances = list_a
            .iter()
            .zip(list_b.iter())
            .map(|(a, b)| (a - b).abs())
            .collect::<Vec<_>>();

        dprintln!("Distances: {:?}", distances);

        let sum: i32 = distances.iter().sum();

        dprintln!("Sum: {}", sum);

        println!("{}", sum);
    }

    #[cfg(feature = "part2")]
    {
        let mut b_list_counts = std::collections::HashMap::new();
        for x in list_b.iter() {
            let count = b_list_counts.entry(*x).or_insert(0);
            *count += 1;
        }

        let mut similarity_list = vec![];
        for x in list_a.iter() {
            let count = b_list_counts.entry(*x).or_insert(0);
            similarity_list.push(*x * *count);
        }

        dprintln!("Similarity List: {:?}", similarity_list);
        let sum: i32 = similarity_list.iter().sum();
        dprintln!("Sum: {}", sum);
        println!("{}", sum);
    }
}

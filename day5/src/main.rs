use std::{
    clone,
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        {
            println!($($arg)*);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    // X must come before Y
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct PageRules {
    rules: Vec<Rule>,
}

impl PageRules {
    fn new() -> Self {
        PageRules { rules: Vec::new() }
    }

    fn parse_rule(&mut self, rule: &str) {
        let mut rule = rule.split('|');
        let x = rule.next().unwrap().parse::<i32>().unwrap();
        let y = rule.next().unwrap().parse::<i32>().unwrap();
        self.rules.push(Rule { x, y });
    }
}

#[derive(Debug)]
struct OptimizedPageRules {
    rules: PageRules,
    y_to_x: HashMap<i32, Vec<Rule>>,
}

impl OptimizedPageRules {
    fn new(rules: PageRules) -> Self {
        let mut y_to_x = HashMap::new();
        for rule in &rules.rules {
            y_to_x
                .entry(rule.y)
                .or_insert(Vec::new())
                .push(rule.clone());
        }
        OptimizedPageRules { rules, y_to_x }
    }
}

#[derive(Debug, Clone)]
struct PageList {
    pages: Vec<i32>,
}

impl PageList {
    fn parse(pages: &str) -> Self {
        let pages = pages
            .split(',')
            .map(|page| page.trim().parse::<i32>().unwrap())
            .collect();
        PageList { pages }
    }

    fn is_valid_page(
        pages: &Vec<i32>,
        page: i32,
        optimized_rules: &OptimizedPageRules,
        visited: &HashSet<i32>,
    ) -> bool {
        for rule in optimized_rules.y_to_x.get(&page).unwrap_or(&vec![]).iter() {
            if !pages.contains(&rule.x) {
                continue;
            }
            if !visited.contains(&rule.x) {
                return false;
            }
        }
        true
    }

    fn is_valid(&self, optimized_rules: &OptimizedPageRules) -> bool {
        let mut visited = HashSet::new();
        for page in &self.pages {
            if !Self::is_valid_page(&self.pages, *page, optimized_rules, &visited) {
                return false;
            }
            visited.insert(*page);
        }
        true
    }

    fn correctly_ordered(&self, optimized_rules: &OptimizedPageRules) -> Self {
        let mut queue = VecDeque::new();
        let mut final_order = vec![];
        let mut visited = HashSet::new();
        for page in &self.pages {
            if !Self::is_valid_page(&self.pages, *page, optimized_rules, &visited) {
                queue.push_back(*page);
            } else {
                final_order.push(*page);
                visited.insert(*page);
            }
        }
        while !queue.is_empty() {
            let page = queue.pop_front().unwrap();
            if Self::is_valid_page(&self.pages, page, optimized_rules, &visited) {
                final_order.push(page);
                visited.insert(page);
            } else {
                queue.push_back(page);
            }
        }
        assert!(final_order.len() == self.pages.len());
        PageList { pages: final_order }
    }
}

fn main() {
    // let mut matrix = WordSearchMatrix::new();
    let mut page_rules = PageRules::new();
    let mut book_list = vec![];
    let mut is_reading_rules = true;
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        for line in lock.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                is_reading_rules = false;
                continue;
            }
            match is_reading_rules {
                true => page_rules.parse_rule(&line),
                false => book_list.push(PageList::parse(&line)),
            }
        }
    }

    dprintln!("Rules: {:?}", page_rules);
    dprintln!("Book List: {:?}", book_list);

    let optimized_rules = OptimizedPageRules::new(page_rules);

    let res1 = book_list
        .iter()
        .filter(|page_list| page_list.is_valid(&optimized_rules))
        .map(|page_list| page_list.pages[page_list.pages.len() / 2])
        .sum::<i32>();

    let res2 = book_list
        .iter()
        .filter(|page_list| !page_list.is_valid(&optimized_rules))
        .map(|page_list| page_list.correctly_ordered(&optimized_rules))
        .collect::<Vec<_>>();

    let res2 = res2
        .iter()
        .map(|page_list| page_list.pages[page_list.pages.len() / 2])
        .sum::<i32>();

    println!("Result: {}", res1);

    println!("Result: {}", res2);
}

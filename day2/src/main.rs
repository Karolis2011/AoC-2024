use std::io::BufRead;

macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        {
            println!($($arg)*);
        }
    }
}

#[derive(Debug)]
struct Report {
    pub levels: Vec<i32>,
}

impl Report {
    
    fn level_deltas_s(levels: &[i32]) -> Vec<i32> {
        levels.iter()
            .zip(levels.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect()
    }

    fn is_safe_s(levels: &[i32]) -> (bool, (bool, bool, bool)) {
        let deltas = Self::level_deltas_s(levels);
        let is_increasing = deltas.iter().all(|&d| d >= 1);
        let is_decreasing = deltas.iter().all(|&d| d <= -1);
        let is_delta_change_safe = deltas.iter().map(|d| d.abs()).all(|d| d >= 1 && d <= 3);
    
        let is_safe = (is_increasing || is_decreasing) && is_delta_change_safe;
        (is_safe, (is_increasing, is_decreasing, is_delta_change_safe))
    }

    fn is_safe(&self) -> (bool, (bool, bool, bool)) {
        Self::is_safe_s(self.levels.as_slice())
    }

    fn is_safe_with_single_ignored(&self) -> bool {
        let (is_safe, _) = Self::is_safe_s(self.levels.as_slice());
        if is_safe {
            return true;
        }
        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);
            let (is_safe, _) = Self::is_safe_s(levels.as_slice());
            if is_safe {
                return true;
            }
        }
        return false;
    }
    
}

fn main() {
    let mut reports = vec![];
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();
        // Read input line by line
        for line in lock.lines() {
            let levels = line.unwrap()
                .split(char::is_whitespace)
                .filter(|s| !s.is_empty())
                .map(|s| i32::from_str_radix(s, 10).unwrap())
                .collect::<Vec<_>>();
            reports.push(Report { levels });
        }
    }
    dprintln!("Reports: {:?}", reports);

    let safe_reports = reports.iter().filter(|r| r.is_safe().0).count();
    let safe_reports_with_single_ignored = reports.iter().filter(|r| r.is_safe_with_single_ignored()).count();

    
    println!("Safe reports: {}", safe_reports);
    println!("Safe reports with single ignored: {}", safe_reports_with_single_ignored);
}

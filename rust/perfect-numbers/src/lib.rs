#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else if num == 1 {
        return Some(Classification::Deficient);
    }

    let max = (num as f64).sqrt() as u64;
    let aliquot_sum = (2..=max).fold(1, |sum, i| {
        if num % i == 0 {
            sum + i + if i == num / i { 0 } else { num / i }
        } else {
            sum
        }
    });
    if aliquot_sum == num {
        Some(Classification::Perfect)
    } else if aliquot_sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}

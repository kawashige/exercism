#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (small, large, swapped) = {
        if first_list.len() < second_list.len() {
            (first_list, second_list, false)
        } else {
            (second_list, first_list, true)
        }
    };

    if small.len() == large.len() && small == large {
        Comparison::Equal
    } else if small.is_empty()
        || (0..(large.len() - small.len()) + 1)
            .filter(|i| small[0] == large[*i])
            .any(|i| &large[i..(small.len() + i)] == small)
    {
        if swapped {
            Comparison::Superlist
        } else {
            Comparison::Sublist
        }
    } else {
        Comparison::Unequal
    }
}

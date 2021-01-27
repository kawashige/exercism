pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut min = 0;
    let mut max = array.len();
    while min < max {
        let mid = (min + max) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            min = mid + 1;
        } else {
            max = mid;
        }
    }
    None
}

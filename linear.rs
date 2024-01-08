
fn linear_search(item: &T, arr: &[T]) -> i32 {
    let mut idx_pos = -1; // -1 indicates not found

    for (idx, data) in arr.iter().enumerate() {
        if item == data {
            idx_pos = idx as i32;
            return idx_pos;
        }
    }
    idx_pos
}

fn binary_search(arr: &[T], target: &T) -> Option {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if &arr[mid] == target {
            return Some(mid); // Target found, return its index
        } else if &arr[mid] < target {
            left = mid + 1; // Search the right half
        } else {
            right = mid; // Search the left half
        }
    }

    None // Target not found, return None
}

fn main() {
    let numbers = vec![1, 4, 6, 8, 10, 25, 42, 53, 64, 72, 87, 99];
    let target = 42;
    let result = binary_search(&numbers, &target);

    match result {
        Some(index) => println!("Found {} at position: {}", target, index),
        None => println!("{} is not present in the array", target),
    }
}
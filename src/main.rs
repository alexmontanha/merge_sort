fn main() {
    let mut arr = [1, 3, 2, 5, 4];
    merge_sort(&mut arr);
    println!("{:?}", arr);
}

fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut ret = Vec::with_capacity(arr.len());
    let (mut i, mut j) = (0, mid);
    while i < mid && j < arr.len() {
        if arr[i] < arr[j] {
            ret.push(arr[i]);
            i += 1;
        } else {
            ret.push(arr[j]);
            j += 1;
        }
    }
    if i < mid {
        ret.extend_from_slice(&arr[i..mid]);
    } else {
        ret.extend_from_slice(&arr[j..]);
    }
    arr.copy_from_slice(&ret);
}

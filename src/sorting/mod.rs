pub fn bubble_sort<T>(arr: &mut Vec<T>, start: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    for i in start..end {
        for j in 0..(end - i) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

pub fn insertion_sort<T>(arr: &mut Vec<T>, start: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    for i in (start + 1)..=end {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}

pub fn selection_sort<T>(arr: &mut Vec<T>, start: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    for i in start..=end {
        let mut min_idx = i;

        for j in (i + 1)..=end {
            if arr[min_idx] > arr[j] {
                min_idx = j
            }
        }

        let temp = arr[min_idx];
        arr[min_idx] = arr[i];
        arr[i] = temp;
    }
}

// first subarray : [start, mid]
// second subarray : (mid, end]
fn merge<T>(arr: &mut Vec<T>, start: usize, mid: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    // temp_arr to keep the combined subarray
    let mut temp_arr = Vec::with_capacity(end - start + 1);

    // indices to keep track of lengths of subarray covered
    let (mut idx1, mut idx2) = (start, mid + 1);

    while idx1 <= mid && idx2 <= end {
        if arr[idx1] > arr[idx2] {
            temp_arr.push(arr[idx2]);
            idx2 += 1;
        } else {
            temp_arr.push(arr[idx1]);
            idx1 += 1;
        }
    }

    // push back the remaining elements;
    // only one of the following loops will run, depending
    // upon which subarray has the largest last element
    while idx1 <= mid {
        temp_arr.push(arr[idx1]);
        idx1 += 1;
    }
    while idx2 <= end {
        temp_arr.push(arr[idx2]);
        idx2 += 1;
    }

    // fill back the elements in the actual arr
    for i in start..=end {
        arr[i] = temp_arr[i - start];
    }
}

#[allow(unused_assignments)]
// first subarray : [start, mid]
// second subarray : (mid, end]
pub fn merge_sort<T>(arr: &mut Vec<T>, start: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    let (mut current_size, length) = (1, arr.len());
    let (mut left_start, mut mid, mut right_end) = (0, 0, 1);

    while current_size <= length / 2 {
        left_start = start;
        mid = start + current_size - 1;
        right_end = mid + current_size;

        while right_end < end {
            merge(arr, left_start, mid, right_end);

            left_start = right_end + 1;
            mid = left_start + current_size - 1;
            right_end = mid + current_size;
        }

        // if mid > end then we know that a single subarray
        // is the remaining array, and mid should be it's mid,
        // not the left out mid
        if mid > end {
            mid = (left_start + end) / 2;
        }

        // the last part or arr intentionally left out
        // from the previous loop, so that we can do merge
        // or remaining length without index out of bound
        merge(arr, left_start, mid, end);

        current_size <<= 1;
    }

    if left_start > 0 {
        merge(arr, start, left_start - 1, end);
    }
}

use crate::heap::Heap;

pub fn heap_sort<T>(arr: &mut Vec<T>, _start: usize, _end: usize)
where
    T: PartialOrd + Copy,
{
    let mut a = Heap::new(arr);

    a.to_max_heap();

    for i in (1..a.len()).rev() {
        a.exchange(0, i);

        a.heap_size -= 1;

        a.max_heapify(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // a single common testing function: DRY
    fn test_this_sort(algo: fn(&mut Vec<u16>, usize, usize)) {
        let arrs = vec![
            vec![324, 132, 97, 324, 897, 324, 324, 7, 96, 239, 864, 3],
            vec![981, 762, 771, 912, 903, 792, 206, 276, 198],
            vec![639, 783, 827, 77, 311, 578, 678, 955],
            vec![615, 740, 30, 206, 8, 750, 345, 911, 789, 763],
            vec![
                473, 553, 245, 688, 524, 430, 794, 202, 337, 128, 954, 749, 540, 152,
            ],
            vec![
                35, 916, 461, 783, 374, 391, 484, 779, 803, 451, 346, 785, 588,
            ],
            vec![430, 339, 855, 818, 807, 486, 585, 692, 568],
            vec![
                158, 562, 427, 185, 646, 807, 592, 279, 645, 541, 579, 707, 16,
            ],
            vec![602, 192, 308, 505, 118, 791, 21],
        ];

        for mut i in arrs {
            let x = i.len() - 1;
            algo(&mut i, 0, x);

            for j in 0..(x - 1) {
                assert!(i[j] <= i[j + 1]);
            }
        }
    }

    #[test]
    fn test_bubble_sort() {
        test_this_sort(bubble_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_this_sort(insertion_sort);
    }

    #[test]
    fn test_selection_sort() {
        test_this_sort(selection_sort);
    }

    #[test]
    fn test_merge_sort() {
        test_this_sort(merge_sort);
    }

    #[test]
    fn test_heap_sort() {
        test_this_sort(heap_sort);
    }
}

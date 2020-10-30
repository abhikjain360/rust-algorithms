#![allow(dead_code)]

use std::ops::Add;

fn max_crossing_subarray<T>(arr: &mut [T], start: usize, mid: usize, end: usize) -> (usize, usize, T)
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    // left extension
    let (mut max_idx, mut sum, mut max_sum) = (mid, arr[mid], arr[mid]);

    for i in (start..mid).rev() {
        sum = sum + arr[i];

        if sum > max_sum {
            max_sum = sum;
            max_idx = i;
        }
    }

    let left = max_idx;
    max_idx = mid;

    for i in (mid + 1)..=end {
        sum = sum + arr[i];

        if sum > max_sum {
            max_sum = sum;
            max_idx = i;
        }
    }

    let right = max_idx;

    (left, right, max_sum)
}

pub fn max_subarray<T>(arr: &mut [T], start: usize, end: usize) -> (usize, usize, T)
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    if start == end {
        return (start, end, arr[start]);
    }

    let mid = (start + end) / 2;

    let left_max = max_subarray(arr, start, mid);
    let mid_max = max_crossing_subarray(arr, start, mid, end);
    let right_max = max_subarray(arr, mid + 1, end);

    if left_max.2 >= mid_max.2 && left_max.2 >= right_max.2 {
        left_max
    } else if mid_max.2 >= right_max.2 {
        mid_max
    } else {
        right_max
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn max_subarray() {
        // TODO: write tests here
    }
}

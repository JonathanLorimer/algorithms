pub fn find_max_crossing_subarray(v: &Vec<i32>, low: usize, mid: usize, high: usize) -> Option<(usize, usize, i32)>  {
    println!("low: {:?}, mid: {:?}, high: {:?}", low, mid, high);
    let mut left_sum: Option<i32> = None;
    let mut right_sum: Option<i32> = None;
    let mut sum: i32 = 0;
    let mut max_left: usize = 0;
    let mut max_right: usize = 0;
    for i in (low..=mid).rev() {
        sum = sum + v[i];
        match left_sum {
            None => {
                left_sum = Some(sum);
                max_left = i;
            },
            Some(ls) => if sum > ls {
                left_sum = Some(sum);
                max_left = i;
            }
        };
    };
    sum = 0;
    for j in (mid + 1)..high {
        sum = sum + v[j];
        match right_sum {
            None => {
                right_sum = Some(sum);
                max_right = j;
            },
            Some(rs) => if sum > rs {
                right_sum = Some(sum);
                max_right = j;
            }
        };

    }
    println!("left_sum: {:?}, right_sum: {:?}, max_left: {:?}, max_right {:?}", left_sum, right_sum, max_left, max_right);
    let res = left_sum.and_then(|ls| right_sum.map(|rs| (max_left, max_right, ls + rs)));
    println!("{:?}", res);
    res
}

pub fn find_max_subarray(v: &Vec<i32>, low: usize, high: usize) -> (usize, usize, i32) {
    if high == low {
        (low, high, v[low])
    } else {
        let mid = (low + high) / 2;
        let (left_low, left_high, left_sum) = find_max_subarray(v, low, mid);
        let (right_low, right_high, right_sum) = find_max_subarray(v, mid + 1, high);
        match find_max_crossing_subarray(v, low, mid, high) {
            Some((cross_low, cross_high, cross_sum)) => {

                if left_sum >= right_sum && left_sum >= cross_sum {
                    (left_low, left_high, left_sum)
                } else if right_sum >= left_sum && right_sum >= cross_sum {
                    (right_low, right_high, right_sum)
                } else {
                    (cross_low, cross_high, cross_sum)
                }
            },
            None => {
                if left_sum >= right_sum {
                    (left_low, left_high, left_sum)
                } else {
                    (right_low, right_high, right_sum)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_subarray_example() {
        let array: Vec<i32> = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        let max_subarray = find_max_subarray(&array, 0, array.len() - 1);
        assert_eq!((7, 10, 43), max_subarray);
    }
}

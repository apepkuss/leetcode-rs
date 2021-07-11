// Copyright 2021 apepkuss
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/* 小红书自动刷题机6.14

Given a list of sorted numbers (can be either negative or positive),
return the list of numbers squared in sorted order.

 */

use std::collections::VecDeque;

pub fn square_nums(nums: &[i32]) -> Vec<i32> {
    let mut res = VecDeque::new();

    if nums.len() == 0 {
        return vec![];
    }

    // 定义双指针指向nums的左右两侧
    let (mut left, mut right) = (0, nums.len() - 1);

    // 比较left和right指向的两个值，直到left和right两个索引错过彼此
    while left <= right {
        let sq_left = i32::pow(nums[left], 2);
        let sq_right = i32::pow(nums[right], 2);

        // 把当前大哥的放到上一轮找到的大哥大的前面
        if sq_left >= sq_right {
            res.push_front(sq_left);
            left += 1;
        } else {
            res.push_front(sq_right);
            right -= 1;
        }
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(square_nums(&nums[..]), vec![1, 4, 9, 16, 25]);

        let nums = vec![-5, -4, -3, -2, -1];
        assert_eq!(square_nums(&nums[..]), vec![1, 4, 9, 16, 25]);

        let nums = vec![-5, -2, 1, 2, 3, 4];
        assert_eq!(square_nums(&nums[..]), vec![1, 4, 4, 9, 16, 25]);

        let nums = vec![-3, 1, 2, 4];
        assert_eq!(square_nums(&nums[..]), vec![1, 4, 9, 16]);

        let nums = vec![0, 0, 0, 0];
        assert_eq!(square_nums(&nums[..]), vec![0, 0, 0, 0]);

        assert_eq!(square_nums(&[]), vec![]);
    }
}

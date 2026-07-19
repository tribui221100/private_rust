use std::ptr::read_unaligned;

pub struct Solution;

impl Solution{

    // Leet3
    // Longest Substring Without Repeating Characters
    pub fn Leet3_LongestSubstring(s: String) -> i32 {
        let len: i32 = s.len() as i32;
        let schar: Vec<char> = s.chars().collect();
        let mut swindow: Vec<char> = Vec::new();
        let mut max_recorded: i32  = 0;
        

        if len == 0  || len == 1 {
            max_recorded = len;
        }
        else {
            let mut curr_s: char = s.chars().nth(0).unwrap();
            swindow.push(schar[0]);
            for i in 0..s.len() {
                if curr_s != schar[i] {
                    if !swindow.contains(&schar[i]) {
                        swindow.push(schar[i]);
                        curr_s = schar[i];
                    }
                } else {
                    if swindow.len() as i32 > max_recorded {
                        max_recorded = swindow.len() as i32;
                    }
                    swindow.clear();
                }
            }   
            if swindow.len() as i32 > max_recorded {
                max_recorded = swindow.len() as i32;
            }
        }

        if swindow.len() as i32 > max_recorded {
            max_recorded = swindow.len() as i32;
        }
        max_recorded
    }


    // Leet 13
    // REQ: Romain to Integers
    pub fn roman_to_value(roman: char) -> i32 {
        match roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0, // Invalid character
        }
    }


    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut result = 0;
        let mut i = 0;

        while i < chars.len() {
            let curr = Self::roman_to_value(chars[i]);

            if i + 1 < chars.len() {
                let next = Self::roman_to_value(chars[i + 1]);

                if curr < next {
                    result += next - curr;
                    i += 2;
                    continue;
                }
            }

            result += curr;
            i += 1;
        }

        result
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut sorted_nums: Vec<i32> = Vec::new();
        let mut key = nums[0];
        sorted_nums = nums.clone();
        for i in 1..nums.len(){
            key = sorted_nums[i];
            let mut j = i;
            while j>0 && key < sorted_nums[j-1]{
                sorted_nums[j] = sorted_nums[j-1];
                j-=1;
            }
            sorted_nums[j] = key;
        }
        println!("Sorted array: {:?}", sorted_nums); 

        for i in 0..sorted_nums.len()-1{
            if sorted_nums[i] == sorted_nums[i+1]{
                return true;
            }
        }
        false
    }
}

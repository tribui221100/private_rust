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
    pub fn Leet13_Romain2Int(s: String) -> i32 {
        0 as i32
    }
}

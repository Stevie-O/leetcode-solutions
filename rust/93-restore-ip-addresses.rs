// 93. Restore IP Addresses
// this version looks like it DOES work

struct Solution {}

fn get_max_octet_length(s : &[u8]) -> usize {
    // okay, so these are the valid octet patterns:
    // [0-9]
    // [1-9][0-9]
    // 1[0-9][0-9]
    // 2[0-4][0-9]
    // 25[0-5]
    // written UNAMBIGUOUSLY, they are:
    // 0
    // 1 ( e | [0-9] ( e | [0-9] ) )
    // 2 ( e | [0-4] ( e | [0-9] ) | 5 ( e | [0-5]) |  [6-9] )
    // [3-9]
    match *s {
        [] => 0,    // shouldn't happen
        [b'0', ..] => 1, // a 0
        [b'1', _, _, ..] => 3,      // a 1 followed by two more digits
        [b'2', b'5', b'0'..=b'5', .. ] => 3, // a 2 followed by a 5 followed by 0-5
        [b'2', b'0'..=b'4', _, ..] => 3,    // a 2 followed by 0-4 followed by another digit
        [b'1'..=b'9', _, ..] => 2,          // a 1-9 followed by any digit
        [b'0'..=b'9'] => 1,                  // any digit
        ref unknown => panic!("unexpected: {:?}", std::str::from_utf8(unknown)),
    }
}

type TrackState = [usize; 4];

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        eprintln!("considering: {s}");
        let b = s.as_bytes();
        let mut queue : Vec<TrackState> = Vec::new();
        queue.push( [0; 4] );
        for index in 0..3  { // 0,1,2
            // we need to leave at least (3-index) digits left at the end for the remaining octets
            let max_next_index = b.len() - (3 - index);
            eprintln!("octet {index}: max_next_index = {max_next_index}");
            let mut next_queue : Vec<TrackState> = Vec::with_capacity(queue.len() * 2);
            for mut indices in queue.into_iter() {
                let start_index = if index == 0 { 0 } else { indices[index - 1] };
                eprintln!("start_index = {start_index} indices = {:?}", &indices[0..index]);
                let max_length = get_max_octet_length(&b[start_index..max_next_index]);
                // with current code logic, max_length should never be ZERO
                eprintln!("max_length = {max_length}");
                if max_length >= 1 {
                    if max_length >= 2 {
                        for len in 2..=max_length {
                            indices[index] = start_index + len;
                            next_queue.push(indices.clone());
                        }
                    }
                    indices[index] = start_index + 1;
                    next_queue.push(indices);
                }
            }
            queue = next_queue;
        }
        eprintln!("final candidate listing: {:?}", queue);
        // okay, at this point, we've picked all of the possible "first three octets".
        // can we place a single octet into the remainder?
        let insert_positions = queue.into_iter().filter_map(|indices| {
                let start_index = indices[2];
                let max_length = get_max_octet_length(&b[start_index..]);
                if start_index + max_length == b.len() {
                    // NOTE **reverse** the order (so we insert at the END)
                    // this simplifies the math 
                    Some([indices[2], indices[1], indices[0]])
                } else {
                    None
                }
        });

        insert_positions.map(|indices| {
            let mut ipstr = Vec::from(b);
            for idx in indices.into_iter() {
                ipstr.insert(idx, b'.');
            }
            String::from_utf8(ipstr).unwrap()
        }).collect()
    }
}

fn test_get_max_octet_length() {
    let mut pass = true;
    for i in 0_u8 ..= 255_u8 {
        let s = i.to_string();
        let expected_len = get_max_octet_length(s.as_bytes());
        if expected_len != s.len() {
            println!("{i} is length {} but get_max_octet_length returned {expected_len}", s.len());
            pass = false;
        }
    }
    if !pass {
        panic!();
    }
}

fn main() {
    test_get_max_octet_length();
    for input in [ 
            "25525511135",
            "0000",
            "101023",
        ]
    {
        println!("input: {}", input);
        let ans = Solution::restore_ip_addresses(String::from(input));
        println!("-> answer: {:?}", ans);
    }
}

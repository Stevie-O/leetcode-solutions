// 93. Restore IP Addresses
// this version doesn't work

fn get_max_octet_length(mut s : &[u8]) -> usize {
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
        [] => 0,
        [b'0', ..] => 1,
        [b'1', _, _, ..] => 3,
        [b'1', _] => 2,
        [b'1'] => 1,
        [b'2', b'5', b'0'..=b'5', .. ] => 3,
        [b'2', b'5', .. ] => 2,
        [b'2', b'0'..=b'4', _, ..] => 3,
        [b'2', b'6'..=b'9', ..] => 2,
        [b'2'] => 1,
        [b'3'..=b'9', ..] => 1,
        ref unknown => panic!("unexpected: {:?}", unknown),
    }
}

use std::collections::HashSet;
type TrackState = ([usize; 4]);

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let b = s.as_bytes();
        let mut queue : Vec<TrackState> = Vec::new();
        queue.push( [0; 4] );
        for index in 0..3  { // 0,1,2
            // we need to leave at least (3-index) digits left at the end for the remaining octets
            let max_next_index = b.len() - (3 - index);
            let mut next_queue : Vec<TrackState> = Vec::with_capacity(queue.len() * 2);
            for mut indices in queue.into_iter() {
                let start_index = if index == 0 { 0 } else { indices[index - 1] };
                let max_length = get_max_octet_length(&b[start_index..max_next_index]);
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
        // okay, at this point, we've picked all of the possible "first three octets".
        // can we place a single octet into the remainder?
        let insert_positions = queue.into_iter().filter_map(|indices| {
                let start_index = indices[3];
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

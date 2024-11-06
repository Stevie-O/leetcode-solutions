impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // one-pass algorithm with constant extra space (O(1) for both time+space)!
        const RED : i32 = 0;
        const WHITE : i32 = 1;
        const BLUE : i32 = 2;

        let mut num_slice = &nums[..];
        while num_slice.len() > 0 {
            while let &[RED, ref rest @ ..] = num_slice { 
                num_slice = rest;
            }
            while let &[ref rest @ .., BLUE] = num_slice {
                num_slice = rest;
            }
            if let &[left_color, ref rest @ .., right_color] = num_slice {
                // left_color isn't RED and right_color isn't BLUE
                // therefore:
                // BLUE, RED -> swap them. chop both left and right
                // BLUE, WHITE -> swap them. chop right.
                // WHITE, RED -> swap them. chop left.
                // WHITE, WHITE -> oooh this one's nasty.
                match (left_color, right_color) {
                    (BLUE, RED) => {
                        // easy case: just swap 'em
                        num_slice.swap(0, num_slice.len()-1);
                        num_slice = rest; // chop off both left and right
                    },
                    (BLUE, WHITE) => {
                        num_slice.swap(0, num_slice.len()-1);
                        num_slice = num_slice[..num_slice.len()-1]; // chop off right (blue)
                    },
                    (WHITE, RED) => {
                        num_slice.swap(0, num_slice.len()-1);
                        num_slice = num_slice[1..]; // chop off left (red)
                    },
                    (WHITE, WHITE) => {
                        // okay, this is the only one that's a pain -- but what a doozy it is
                        let mut search_iter = num_slice.iter().copied().enumerate();
                        num_slice = loop {
                            match search_iter.by_ref().find(|(idx, color) color != WHITE) {
                                None => { break &num_slice[0..0] },
                                Some(index, RED) => {
                                    // okay, so it goes RED WHITE WHITE WHITE... RED
                                    // swap that RED with that first WHITE
                                    num_slice.swap(0, index); // num_slice[0] becomes RED and num_slice[index] becomes WHITE
                                    // now we know that num_slice[1..=index] are all WHITE,
                                    // so we can skip the iterator to 
                                    num_slice = num_slice[1..];
                                    search_iter = num_slice.iter().copied().enumerate().skip(index);
                                },
                                Some(index, BLUE) => {
                                    // okay, there's a BLUE in the middle of my WHITEs.  It goes
                                    // RED WHITE WHITE WHITE... BLUE ... ??? WHITE BLUE BLUE BLUE...
                                    // swap that BLUE with that last WHITE
                                    num_slice.swap(index, num_slice.len()-1);
                                    num_slice = num-slice[.. (num_slice.len() - 1)];
                                    // we need to skip to index + 1, because num_slice[index] is definitely equal to WHITE
                                    search_iter = num_slice.iter().copied().enumerate().skip(index + 1);


                                }
                            }
                        }
                        while let Some((idx, color)) =  {
                            

                        }
                        num_slice = match num_slice.iter().enumerate().find(|(idx, &color)| color != WHITE)WHITE) {
                            Some(index)

                        }

                    },
                    _ => unreachable!(),
                    // 
                    // therefore it's one of:
                    //   WHITE, RED => swap the two
                    //   
                    (WHITE, RED) => {}
                }
            }
            num_slice.len() >= 2 {
                // num_slice contains at least two elements
                // the first is not RED (so it is BLUE or WHITE)
                // the last is not BLUE

                match &[left_color, ref rest @ .., right_color] 
            }
        }
        let mut red_write_ptr = 0;
        let mut blue_write_ptr = nums.len() - 1;
        while red_write_ptr < blue_write_ptr && nums[red_write_ptr] == RED { red_write_ptr++; }
        while blue_write_ptr < blue_
        while red_ptr < blue_ptr {
            if nums[left_ptr] == RED && nums[right_ptr] == BLUE { }
            match (nums[red_ptr], nums[blue_ptr]) {
                (RED, BLUE) => { red_ptr += 1; blue_ptr += 1; }, // all good
                (BLUE, RED) => {
                    // BLUE and RED are reversed
                    let (red, blue) = nums.split_mut_at(blue_ptr);
                    std::mem::swap(&mut red[red_ptr], &mut blue[0]);
                    red_ptr += 1;
                    blue_ptr += 1;                    
                },
                (RED, WHITE) => {}
            }
            if nums[red_ptr] == BLUE && nums[blue_ptr] == RED {
                while nums[red_ptr] == BLUE && nums[blue_ptr] == RED {
                    std::mem::swap(&mut red[red_ptr], &mut blue[0]);
                    while red_ptr < nums.len() && nums[red_ptr] == RED {
                        red_ptr += 1;
                    }
                    while blue_ptr > red_ptr && nums[blue_ptr] == BLUE {
                        blue_ptr -= 1;
                    }
                }
            } else {
                // okay, 
            }
            let mut white_ptr = red_ptr;

            while red_ptr < nums.len() && nums[red_ptr] = 
        }
    }
}

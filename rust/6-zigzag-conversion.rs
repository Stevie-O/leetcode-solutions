fn calc_zigzag(row : usize, rows : usize) -> (usize, usize) {
    let lastRow = rows - 1;
    // the first and last rows behave _differently_. Technically one of these is _zero_ but if we pretend it's the other one,
	// the math works out
    match row {
        r if r == 0 || r == lastRow => 
            (2 * lastRow, 2 * lastRow), // it's actually rows + (rows - 2) but that's 2*rows-2 which is 2*(rows-1) which is 2*lastRow
        row => (
            // how long does it take to get down to the bottom and hit this same row on the way back up?
            (lastRow - row) * 2,
            // how long does it take to get up to the top and hit this same row on the way back down?
            (row) * 2            
        ),
    }
}

impl Solution {

    pub fn convert(s: String, num_rows: i32) -> String {

        let num_rows = num_rows as usize;
        let in_buf = s.as_bytes();

        // degenerate cases:
        //  - only 1 row: s is unchanged
        //  - too many rows: s just gets drawn vertically
        if num_rows < 2 || (num_rows as usize) >= in_buf.len() { return s; }

        let mut out_buf : Vec<u8> = Vec::with_capacity(in_buf.len());
    
        for row in (0..num_rows) {
            let (downup, updown) = calc_zigzag(row, num_rows);
    		// we start by moving DOWN, so the first character on row n is the character originally at position n in the original string
		    let mut pos = row as usize;
            out_buf.push(in_buf[pos]);
            let buflen = in_buf.len();
            while (pos < buflen)
            {
                pos += downup; // go down to the bottom and back up to this row
                if (pos >= buflen) { break; }
                out_buf.push(in_buf[pos]);
                pos += updown; // go up to the top and back down to this row
                if (pos >= buflen) { break; }
                out_buf.push(in_buf[pos]);
            }
	    }

        String::from_utf8(out_buf).unwrap()
    }
}

struct Solution {}

struct PunchedWord
{
    word : &[u8],
    punched_index : usize,
}

impl std::hash::Hash for PunchedWord {
    fn hash<H>(&self, state: &mut H)
       where H: Hasher
   {
        let pi = self.punched_index;
        if pi > 0 { state.write( self.word[0..pi] ); }
        if pi < self.word.len() - 1 { state.write( self.word[pi+1 .. ]); }
   }
}

impl PartialEq for PunchedWord {
    fn eq(&self, other: &PunchedWord) -> bool {
        let pi = self.punched_index;
        pi == other.punched_index
        && (pi == 0 || self.word[0..pi] == other.word[0..pi])
        && (pi == self.word.len() - 1 || self.word[pi+1..] == other.word[pi+1..])
    }
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {

        fn make_comparer(word_list: &Vec<String>, index : usize) -> _ {
            |a, b| {
                let word1 = word_list[a].as_bytes();
                let word2 = word_list[b].as_bytes();
                if index > 0 {
                    let ord = word1[0..index].cmp(word2[0..idx]);
                    if ord != Ordering::Equal { return ord; }
                }
                if index < word1.len() - 1 {
                    word1[(index+1)..].cmp(word2[(index+1)..])
                } else {
                    Ordering::Equal
                }
            }
        }

        let word_len = begin_word.len();
        let begin_idx = word_list.len();
        word_list.push(begin_word);
        let end_idx = word_list.len();
        word_list.push(end_word);
        let word_list = word_list; // strip 'mut' from word_list
        let mut graph : HashMap<usize, Vec<usize>> =
            (0 .. word_list.len()).map(|id| (id, Default::default())).collect();
        let mut word_order = (0 .. word_list.len()).to_vec();
        for idx in 0 .. word_len {
            let comparer = make_comparer(&word_list, idx);
            word_order.sort_unstable_by(comparer);

        }

        let short_dist = get_dist(begin_word.as_str(), end_word.as_str());
        if short_dist == 1 { return 1; }
        let mut end_list   = word_list.map(|s| s.as_str()).collect();
        let mut begin_list = end_list.clone();
        end_list.push(begin_word.as_str());
        begin_list.push(end_word.as_str());
    }
}

fn main() {
    let beginWord : String = "hit";
    let endWord: String = "cog";
    let wordList: Vec<String> = ["hot","dot","dog","lot","log","cog"].iter().map(String::to_string);
    let ans = Solution::find_ladders(beginWord, endWord, wordList);
    println!("ans = {ans}");
}

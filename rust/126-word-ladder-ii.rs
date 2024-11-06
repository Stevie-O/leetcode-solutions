/* this is the 2nd version which contains an optimization to reduce memory usage
 * it *still* OOMed, but this time on a different input (number 34 of 37)
 * begin="aaaaa", end="ggggg", wordList=["aaaaa","caaaa","cbaaa","daaaa","dbaaa","eaaaa","ebaaa","faaaa","fbaaa","gaaaa","gbaaa","haaaa","hbaaa","iaaaa","ibaaa","jaaaa","jbaaa","kaaaa","kbaaa","laaaa","lbaaa","maaaa","mbaaa","naaaa","nbaaa","oaaaa","obaaa","paaaa","pbaaa","bbaaa","bbcaa","bbcba","bbdaa","bbdba","bbeaa","bbeba","bbfaa","bbfba","bbgaa","bbgba","bbhaa","bbhba","bbiaa","bbiba","bbjaa","bbjba","bbkaa","bbkba","bblaa","bblba","bbmaa","bbmba","bbnaa","bbnba","bboaa","bboba","bbpaa","bbpba","bbbba","abbba","acbba","dbbba","dcbba","ebbba","ecbba","fbbba","fcbba","gbbba","gcbba","hbbba","hcbba","ibbba","icbba","jbbba","jcbba","kbbba","kcbba","lbbba","lcbba","mbbba","mcbba","nbbba","ncbba","obbba","ocbba","pbbba","pcbba","ccbba","ccaba","ccaca","ccdba","ccdca","cceba","cceca","ccfba","ccfca","ccgba","ccgca","cchba","cchca","cciba","ccica","ccjba","ccjca","cckba","cckca","cclba","cclca","ccmba","ccmca","ccnba","ccnca","ccoba","ccoca","ccpba","ccpca","cccca","accca","adcca","bccca","bdcca","eccca","edcca","fccca","fdcca","gccca","gdcca","hccca","hdcca","iccca","idcca","jccca","jdcca","kccca","kdcca","lccca","ldcca","mccca","mdcca","nccca","ndcca","occca","odcca","pccca","pdcca","ddcca","ddaca","ddada","ddbca","ddbda","ddeca","ddeda","ddfca","ddfda","ddgca","ddgda","ddhca","ddhda","ddica","ddida","ddjca","ddjda","ddkca","ddkda","ddlca","ddlda","ddmca","ddmda","ddnca","ddnda","ddoca","ddoda","ddpca","ddpda","dddda","addda","aedda","bddda","bedda","cddda","cedda","fddda","fedda","gddda","gedda","hddda","hedda","iddda","iedda","jddda","jedda","kddda","kedda","lddda","ledda","mddda","medda","nddda","nedda","oddda","oedda","pddda","pedda","eedda","eeada","eeaea","eebda","eebea","eecda","eecea","eefda","eefea","eegda","eegea","eehda","eehea","eeida","eeiea","eejda","eejea","eekda","eekea","eelda","eelea","eemda","eemea","eenda","eenea","eeoda","eeoea","eepda","eepea","eeeea","ggggg","agggg","ahggg","bgggg","bhggg","cgggg","chggg","dgggg","dhggg","egggg","ehggg","fgggg","fhggg","igggg","ihggg","jgggg","jhggg","kgggg","khggg","lgggg","lhggg","mgggg","mhggg","ngggg","nhggg","ogggg","ohggg","pgggg","phggg","hhggg","hhagg","hhahg","hhbgg","hhbhg","hhcgg","hhchg","hhdgg","hhdhg","hhegg","hhehg","hhfgg","hhfhg","hhigg","hhihg","hhjgg","hhjhg","hhkgg","hhkhg","hhlgg","hhlhg","hhmgg","hhmhg","hhngg","hhnhg","hhogg","hhohg","hhpgg","hhphg","hhhhg","ahhhg","aihhg","bhhhg","bihhg","chhhg","cihhg","dhhhg","dihhg","ehhhg","eihhg","fhhhg","fihhg","ghhhg","gihhg","jhhhg","jihhg","khhhg","kihhg","lhhhg","lihhg","mhhhg","mihhg","nhhhg","nihhg","ohhhg","oihhg","phhhg","pihhg","iihhg","iiahg","iiaig","iibhg","iibig","iichg","iicig","iidhg","iidig","iiehg","iieig","iifhg","iifig","iighg","iigig","iijhg","iijig","iikhg","iikig","iilhg","iilig","iimhg","iimig","iinhg","iinig","iiohg","iioig","iiphg","iipig","iiiig","aiiig","ajiig","biiig","bjiig","ciiig","cjiig","diiig","djiig","eiiig","ejiig","fiiig","fjiig","giiig","gjiig","hiiig","hjiig","kiiig","kjiig","liiig","ljiig","miiig","mjiig","niiig","njiig","oiiig","ojiig","piiig","pjiig","jjiig","jjaig","jjajg","jjbig","jjbjg","jjcig","jjcjg","jjdig","jjdjg","jjeig","jjejg","jjfig","jjfjg","jjgig","jjgjg","jjhig","jjhjg","jjkig","jjkjg","jjlig","jjljg","jjmig","jjmjg","jjnig","jjnjg","jjoig","jjojg","jjpig","jjpjg","jjjjg","ajjjg","akjjg","bjjjg","bkjjg","cjjjg","ckjjg","djjjg","dkjjg","ejjjg","ekjjg","fjjjg","fkjjg","gjjjg","gkjjg","hjjjg","hkjjg","ijjjg","ikjjg","ljjjg","lkjjg","mjjjg","mkjjg","njjjg","nkjjg","ojjjg","okjjg","pjjjg","pkjjg","kkjjg","kkajg","kkakg","kkbjg","kkbkg","kkcjg","kkckg","kkdjg","kkdkg","kkejg","kkekg","kkfjg","kkfkg","kkgjg","kkgkg","kkhjg","kkhkg","kkijg","kkikg","kkljg","kklkg","kkmjg","kkmkg","kknjg","kknkg","kkojg","kkokg","kkpjg","kkpkg","kkkkg","ggggx","gggxx","ggxxx","gxxxx","xxxxx","xxxxy","xxxyy","xxyyy","xyyyy","yyyyy","yyyyw","yyyww","yywww","ywwww","wwwww","wwvww","wvvww","vvvww","vvvwz","avvwz","aavwz","aaawz","aaaaz"]
 *
 * hrm.  this also failed in the playground (SIGKILL).
 * hrrrm. It looks like it was building that Vec for the word list that was problematic
 */
 
struct Solution{}

use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::fmt::{Debug, Formatter, Error};

#[derive(Eq)]
struct PunchedWord<'a>
{
    word: &'a [u8],
    punched_index : usize,
}

impl<'a> PunchedWord<'a> {
    pub fn new(word : &'a String, punched_index : usize) -> Self {
        PunchedWord { word: word.as_bytes(), punched_index: punched_index }
    }
}

impl<'a> Hash for PunchedWord<'a> {
    fn hash<H>(&self, state: &mut H)
       where H: Hasher
   {
        let pi = self.punched_index;
        if pi > 0 { state.write( &self.word[0..pi] ); }
        if pi < self.word.len() - 1 { state.write( &self.word[pi+1 .. ]); }
   }
}

impl<'a> PartialEq for PunchedWord<'a> {
    fn eq(&self, other: &PunchedWord) -> bool {
        let pi = self.punched_index;
        pi == other.punched_index
        && (pi == 0 || self.word[0..pi] == other.word[0..pi])
        && (pi == self.word.len() - 1 || self.word[pi+1..] == other.word[pi+1..])
    }
}

#[derive(Default, Clone)]
struct WordPath {
    order: Vec<usize>,
    visited: HashSet<usize>,
}

impl Debug for WordPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { self.order.fmt(f) }
}

impl WordPath {
    pub fn end(&self) -> usize { self.order[self.order.len()-1] }
    pub fn is_visited(&self, id : usize) -> bool { self.visited.contains(&id) }
    pub fn visit(&mut self, id : usize) {
        self.order.push(id);
        self.visited.insert(id);
    }
    pub fn new(id: usize) -> Self {
        WordPath {
            order: vec![ id ],
            visited: { let mut h = HashSet::new(); h.insert(id); h },
        }
    }
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        let word_len = begin_word.len();
        // this isn't really spelled out, but end_word must be in the word list, while begin_word might not be
        let end_id =
            word_list.iter().enumerate().find(|(_, word)| *word == &end_word).map(|(id, _)| id);
        let end_id = match end_id {
            Some(id) => id,
            None => { return Default::default(); }
        };
        // add begin_word to word_list
        let begin_id = word_list.len();
        word_list.push(begin_word);
        //let end_id = word_list.len();
        //word_list.push(end_word);
        let word_list = word_list; // strip 'mut' from word_list
        /*
        let mut begin_id : Option<usize> = None;
        let mut end_id   : Option<usize> = None;
        for (id, word) in word_list.iter().enumerate() {
            println!("checking {word} against {begin_word}");
            if begin_id.is_none() && word == &begin_word {
                begin_id = Some(id);
                if end_id.is_some() { break; }
            }
            else if end_id.is_none() && word == &end_word {
                end_id = Some(id);
                if begin_id.is_some() { break; }
            }
        }
        let begin_id = begin_id.unwrap();
        let end_id   = end_id.unwrap();
        */
        // build the graph: time complexity is roughly O(mn + (n')^2) where:
        //   m = word length
        //   n = number of words in wordlist
        //   n' = largest number of words that are exactly one letter off from one another
        // this assumes O(1) on hashmap insert/lookup
        let mut graph : Vec<Vec<usize>> = word_list.iter().map(|_| Default::default()).collect();
        for idx in 0 .. word_len {
            let mut linkmap : HashMap<PunchedWord, Vec<usize>> = HashMap::new();
            // find all sets of words that match EXCEPT for the character at position [idx]
            for (word_id, word) in word_list.iter().enumerate() {
                let pw = PunchedWord::new(word, idx);
                linkmap.entry(pw).and_modify(|v| v.push(word_id)).or_insert(vec![word_id]);
            }
            // all words that in the same vec are one step away from another
            // all words in different vecs are more than one step away from another
            for chain_list in linkmap.values() {
                //println!("words linked by index {}: {:?}", idx, chain_list.iter().map(|&id| &word_list[id]).collect::<Vec<_>>());
                // note that this '..' EXCLUDES the last element of chain_list from w1 
                for (chain_idx, w1) in chain_list[0..chain_list.len()-1].iter().enumerate() {
                    for w2 in &chain_list[chain_idx+1..] {
                        // there is an edge from w1->w2
                        graph[*w1].push(*w2);
                        // and there is an edge from w2->w1
                        graph[*w2].push(*w1);
                    }
                }
            }
        }
        let graph = graph; // strip 'mut' from graph
        // at this point, graph[x] should be a vec with all words reachable from x
        
        let mut chains = vec![ WordPath::new(begin_id) ];
        let mut reached_end = false;
        let mut reached : HashSet<usize> = HashSet::new();
        reached.insert(begin_id);
        
        //println!("chains: {:?}", chains);
        
        // breadth-first search to reach end_id
        while !reached_end && chains.len() > 0 {
            let mut next_chains : Vec<WordPath> = Vec::new();
            let mut next_reached : HashSet<usize> = HashSet::new();

            for mut chain in chains {
                let mut buffered_next_id : Option<usize> = None;
                // I cribbed this from the Linux bridge code
                // basically, we optimize for the case where there is only one valid next step
                // the graph was initialized with empty vectors, so graph.get() should never fail
                let links = &graph[chain.end()];
                //println!("links: {:?}", links);
                for &link in links.iter().filter(|&id| !reached.contains(id)) {
                    next_reached.insert(link);
                    if link == end_id { reached_end = true; }
                    if let Some(prev_buffered_id) = buffered_next_id.replace(link) {
                        let mut newchain = chain.clone();
                        newchain.visit(prev_buffered_id);
                        //println!("considering: {:?}", newchain);
                        next_chains.push(newchain);
                    }
                }
                if let Some(prev_buffered_id) = buffered_next_id.take() {
                    chain.visit(prev_buffered_id);
                    //println!("considering: {:?}", chain);
                    next_chains.push(chain);
                }
            }
            reached.extend(next_reached.iter());
            // move over one
            chains = next_chains;
        }
        chains
            .into_iter()
            .filter(|wp| wp.end() == end_id)
            .map(|wp| wp.order.into_iter().map(|id| word_list[id].clone()).collect())
            .collect()
    }
}


fn main() {
    let begin_word = String::from("aaaaa"); // "hit"
    let end_word = String::from("ggggg"); // "cog"
    let word_list: Vec<String> =
        //["hot","dot","dog","lot","log","cog"]
        ["aaaaa","caaaa","cbaaa","daaaa","dbaaa","eaaaa","ebaaa","faaaa","fbaaa","gaaaa","gbaaa","haaaa","hbaaa","iaaaa","ibaaa","jaaaa","jbaaa","kaaaa","kbaaa","laaaa","lbaaa","maaaa","mbaaa","naaaa","nbaaa","oaaaa","obaaa","paaaa","pbaaa","bbaaa","bbcaa","bbcba","bbdaa","bbdba","bbeaa","bbeba","bbfaa","bbfba","bbgaa","bbgba","bbhaa","bbhba","bbiaa","bbiba","bbjaa","bbjba","bbkaa","bbkba","bblaa","bblba","bbmaa","bbmba","bbnaa","bbnba","bboaa","bboba","bbpaa","bbpba","bbbba","abbba","acbba","dbbba","dcbba","ebbba","ecbba","fbbba","fcbba","gbbba","gcbba","hbbba","hcbba","ibbba","icbba","jbbba","jcbba","kbbba","kcbba","lbbba","lcbba","mbbba","mcbba","nbbba","ncbba","obbba","ocbba","pbbba","pcbba","ccbba","ccaba","ccaca","ccdba","ccdca","cceba","cceca","ccfba","ccfca","ccgba","ccgca","cchba","cchca","cciba","ccica","ccjba","ccjca","cckba","cckca","cclba","cclca","ccmba","ccmca","ccnba","ccnca","ccoba","ccoca","ccpba","ccpca","cccca","accca","adcca","bccca","bdcca","eccca","edcca","fccca","fdcca","gccca","gdcca","hccca","hdcca","iccca","idcca","jccca","jdcca","kccca","kdcca","lccca","ldcca","mccca","mdcca","nccca","ndcca","occca","odcca","pccca","pdcca","ddcca","ddaca","ddada","ddbca","ddbda","ddeca","ddeda","ddfca","ddfda","ddgca","ddgda","ddhca","ddhda","ddica","ddida","ddjca","ddjda","ddkca","ddkda","ddlca","ddlda","ddmca","ddmda","ddnca","ddnda","ddoca","ddoda","ddpca","ddpda","dddda","addda","aedda","bddda","bedda","cddda","cedda","fddda","fedda","gddda","gedda","hddda","hedda","iddda","iedda","jddda","jedda","kddda","kedda","lddda","ledda","mddda","medda","nddda","nedda","oddda","oedda","pddda","pedda","eedda","eeada","eeaea","eebda","eebea","eecda","eecea","eefda","eefea","eegda","eegea","eehda","eehea","eeida","eeiea","eejda","eejea","eekda","eekea","eelda","eelea","eemda","eemea","eenda","eenea","eeoda","eeoea","eepda","eepea","eeeea","ggggg","agggg","ahggg","bgggg","bhggg","cgggg","chggg","dgggg","dhggg","egggg","ehggg","fgggg","fhggg","igggg","ihggg","jgggg","jhggg","kgggg","khggg","lgggg","lhggg","mgggg","mhggg","ngggg","nhggg","ogggg","ohggg","pgggg","phggg","hhggg","hhagg","hhahg","hhbgg","hhbhg","hhcgg","hhchg","hhdgg","hhdhg","hhegg","hhehg","hhfgg","hhfhg","hhigg","hhihg","hhjgg","hhjhg","hhkgg","hhkhg","hhlgg","hhlhg","hhmgg","hhmhg","hhngg","hhnhg","hhogg","hhohg","hhpgg","hhphg","hhhhg","ahhhg","aihhg","bhhhg","bihhg","chhhg","cihhg","dhhhg","dihhg","ehhhg","eihhg","fhhhg","fihhg","ghhhg","gihhg","jhhhg","jihhg","khhhg","kihhg","lhhhg","lihhg","mhhhg","mihhg","nhhhg","nihhg","ohhhg","oihhg","phhhg","pihhg","iihhg","iiahg","iiaig","iibhg","iibig","iichg","iicig","iidhg","iidig","iiehg","iieig","iifhg","iifig","iighg","iigig","iijhg","iijig","iikhg","iikig","iilhg","iilig","iimhg","iimig","iinhg","iinig","iiohg","iioig","iiphg","iipig","iiiig","aiiig","ajiig","biiig","bjiig","ciiig","cjiig","diiig","djiig","eiiig","ejiig","fiiig","fjiig","giiig","gjiig","hiiig","hjiig","kiiig","kjiig","liiig","ljiig","miiig","mjiig","niiig","njiig","oiiig","ojiig","piiig","pjiig","jjiig","jjaig","jjajg","jjbig","jjbjg","jjcig","jjcjg","jjdig","jjdjg","jjeig","jjejg","jjfig","jjfjg","jjgig","jjgjg","jjhig","jjhjg","jjkig","jjkjg","jjlig","jjljg","jjmig","jjmjg","jjnig","jjnjg","jjoig","jjojg","jjpig","jjpjg","jjjjg","ajjjg","akjjg","bjjjg","bkjjg","cjjjg","ckjjg","djjjg","dkjjg","ejjjg","ekjjg","fjjjg","fkjjg","gjjjg","gkjjg","hjjjg","hkjjg","ijjjg","ikjjg","ljjjg","lkjjg","mjjjg","mkjjg","njjjg","nkjjg","ojjjg","okjjg","pjjjg","pkjjg","kkjjg","kkajg","kkakg","kkbjg","kkbkg","kkcjg","kkckg","kkdjg","kkdkg","kkejg","kkekg","kkfjg","kkfkg","kkgjg","kkgkg","kkhjg","kkhkg","kkijg","kkikg","kkljg","kklkg","kkmjg","kkmkg","kknjg","kknkg","kkojg","kkokg","kkpjg","kkpkg","kkkkg","ggggx","gggxx","ggxxx","gxxxx","xxxxx","xxxxy","xxxyy","xxyyy","xyyyy","yyyyy","yyyyw","yyyww","yywww","ywwww","wwwww","wwvww","wvvww","vvvww","vvvwz","avvwz","aavwz","aaawz","aaaaz"]
        .iter().map(|s| s.to_string()).collect();
    let ans = Solution::find_ladders(begin_word, end_word, word_list);
    println!("number of solutions: {}", ans.len());
    for sln in ans {
        println!("{:?}", sln);
    }
}

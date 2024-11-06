// version 2.2, which uses a newtype wrapper around (usize) for word IDs
// but is otherwise the same as 1.1

use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::fmt::{Debug, Formatter, Error};

struct Solution {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct WordId(usize);
impl From<usize> for WordId { fn from(value : usize) -> Self { WordId(value) } }
impl From<WordId> for usize { fn from(value: WordId) -> Self { value.0 } }
impl WordId {
    pub fn to_string<'a>(&self, words: &'a Vec<String>) -> &'a String { &words[self.0] }
}

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
    order: Vec<WordId>,
    //visited: HashSet<usize>,
}

impl Debug for WordPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { (self.order).fmt(f) }
}

impl WordPath {
    pub fn end(&self) -> WordId { self.order[self.order.len()-1] }
    //pub fn is_visited(&self, id : usize) -> bool { self.visited.contains(&id) }
    pub fn visit(&mut self, id : WordId) {
        self.order.push(id);
        //self.visited.insert(id);
    }
    pub fn new(id: WordId) -> Self {
        WordPath {
            order: vec![ id ],
            //visited: { let mut h = HashSet::new(); h.insert(id); h },
        }
    }
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        let word_len = begin_word.len();
        // this isn't really spelled out, but end_word must be in the word list, while begin_word **might** not be
        let end_id =
            word_list.iter().enumerate().find(|(_, word)| *word == &end_word).map(|(id, _)| id);
        let end_id : WordId = match end_id {
            Some(id) => id,
            None => { return Default::default(); }
        }
        .into();
        // find begin_id. TODO: figure out how to do this and end_id in one_pass
        let begin_id : WordId = match
            word_list.iter().enumerate().find(|(_, word)| *word == &begin_word).map(|(id, _)| id)
        {
            Some(id) => id,
            None => {
                // add begin_word to word_list
                let id = word_list.len();
                word_list.push(begin_word);
                id
            }
        }
        .into();
        let word_list = word_list; // strip 'mut' from word_list
        //eprintln!("number of words in word list: {}", word_list.len());
        // build the graph: time complexity is roughly O(mn + (n')^2) where:
        //   m = word length
        //   n = number of words in wordlist
        //   n' = largest number of words that are exactly one letter off from one another
        // this assumes O(1) on hashmap insert/lookup
        let mut graph : Vec<Vec<WordId>> = word_list.iter().map(|_| Default::default()).collect();
        for idx in 0 .. word_len {
            let mut linkmap : HashMap<PunchedWord, Vec<WordId>> = HashMap::new();
            // find all sets of words that match EXCEPT for the character at position [idx]
            // Rustiness: should I use map here instead of fixing up word_id in the loop body?
            for (word_id, word) in word_list.iter().enumerate() {
                let word_id : WordId = word_id.into();
                let pw = PunchedWord::new(word, idx);
                linkmap.entry(pw).and_modify(|v| v.push(word_id)).or_insert(vec![word_id]);
            }
            // all words that in the same vec are one step away from another
            // all words in different vecs are more than one step away from another
            for chain_list in linkmap.values().filter(|c| c.len() > 1) {
                //eprintln!("words linked by index {}: {:?}", idx, chain_list.iter().map(|&id| &word_list[id]).collect::<Vec<_>>());
                // note that this '..' EXCLUDES the last element of chain_list from w1 
                for (chain_idx, w1) in chain_list[0..chain_list.len()-1].iter().enumerate() {
                    for w2 in &chain_list[chain_idx+1..] {
                        // there is an edge from w1->w2
                        graph[w1.0].push(*w2);
                        // and there is an edge from w2->w1
                        graph[w2.0].push(*w1);
                    }
                }
            }
        }
        let graph = graph; // strip 'mut' from graph
        // at this point, graph[x] should be a vec with all words reachable from x
        
        let mut chains = vec![ WordPath::new(begin_id) ];
        let mut reached_end = false;
        let mut reached : HashSet<_> = HashSet::new();
        reached.insert(begin_id);
        
        //eprintln!("chains: {:?}", chains);
        
        // breadth-first search to reach end_id
        while !reached_end && chains.len() > 0 {
            let mut next_chains : Vec<WordPath> = Vec::new();
            let mut next_reached : HashSet<_> = HashSet::new();

            for mut chain in chains {
                let mut buffered_next_id : Option<WordId> = None;
                // I cribbed this from the Linux bridge code
                // basically, we optimize for the case where there is only one valid next step
                // the graph was initialized with empty vectors, so graph.get() should never fail
                let links = &graph[chain.end().0];
                //eprintln!("links: {:?}", links);
                for &link in links.iter().filter(|&id| !reached.contains(id)) {
                    next_reached.insert(link);
                    if link == end_id { reached_end = true; }
                    if let Some(prev_buffered_id) = buffered_next_id.replace(link) {
                        let mut newchain = chain.clone();
                        newchain.visit(prev_buffered_id);
                        //eprintln!("considering: {:?}", newchain);
                        next_chains.push(newchain);
                    }
                }
                if let Some(prev_buffered_id) = buffered_next_id.take() {
                    chain.visit(prev_buffered_id);
                    //eprintln!("considering: {:?}", chain);
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
            .map(|wp| wp.order.into_iter().map(|id| id.to_string(&word_list).clone()).collect())
            .collect()
    }
}

const BEGIN_WORD : &str = "cet";
const END_WORD   : &str = "ism";
const WORD_LIST  : &[&str] = &["kid","tag","pup","ail","tun","woo","erg","luz","brr","gay","sip","kay","per","val","mes","ohs","now","boa","cet","pal","bar","die","war","hay","eco","pub","lob","rue","fry","lit","rex","jan","cot","bid","ali","pay","col","gum","ger","row","won","dan","rum","fad","tut","sag","yip","sui","ark","has","zip","fez","own","ump","dis","ads","max","jaw","out","btu","ana","gap","cry","led","abe","box","ore","pig","fie","toy","fat","cal","lie","noh","sew","ono","tam","flu","mgm","ply","awe","pry","tit","tie","yet","too","tax","jim","san","pan","map","ski","ova","wed","non","wac","nut","why","bye","lye","oct","old","fin","feb","chi","sap","owl","log","tod","dot","bow","fob","for","joe","ivy","fan","age","fax","hip","jib","mel","hus","sob","ifs","tab","ara","dab","jag","jar","arm","lot","tom","sax","tex","yum","pei","wen","wry","ire","irk","far","mew","wit","doe","gas","rte","ian","pot","ask","wag","hag","amy","nag","ron","soy","gin","don","tug","fay","vic","boo","nam","ave","buy","sop","but","orb","fen","paw","his","sub","bob","yea","oft","inn","rod","yam","pew","web","hod","hun","gyp","wei","wis","rob","gad","pie","mon","dog","bib","rub","ere","dig","era","cat","fox","bee","mod","day","apr","vie","nev","jam","pam","new","aye","ani","and","ibm","yap","can","pyx","tar","kin","fog","hum","pip","cup","dye","lyx","jog","nun","par","wan","fey","bus","oak","bad","ats","set","qom","vat","eat","pus","rev","axe","ion","six","ila","lao","mom","mas","pro","few","opt","poe","art","ash","oar","cap","lop","may","shy","rid","bat","sum","rim","fee","bmw","sky","maj","hue","thy","ava","rap","den","fla","auk","cox","ibo","hey","saw","vim","sec","ltd","you","its","tat","dew","eva","tog","ram","let","see","zit","maw","nix","ate","gig","rep","owe","ind","hog","eve","sam","zoo","any","dow","cod","bed","vet","ham","sis","hex","via","fir","nod","mao","aug","mum","hoe","bah","hal","keg","hew","zed","tow","gog","ass","dem","who","bet","gos","son","ear","spy","kit","boy","due","sen","oaf","mix","hep","fur","ada","bin","nil","mia","ewe","hit","fix","sad","rib","eye","hop","haw","wax","mid","tad","ken","wad","rye","pap","bog","gut","ito","woe","our","ado","sin","mad","ray","hon","roy","dip","hen","iva","lug","asp","hui","yak","bay","poi","yep","bun","try","lad","elm","nat","wyo","gym","dug","toe","dee","wig","sly","rip","geo","cog","pas","zen","odd","nan","lay","pod","fit","hem","joy","bum","rio","yon","dec","leg","put","sue","dim","pet","yaw","nub","bit","bur","sid","sun","oil","red","doc","moe","caw","eel","dix","cub","end","gem","off","yew","hug","pop","tub","sgt","lid","pun","ton","sol","din","yup","jab","pea","bug","gag","mil","jig","hub","low","did","tin","get","gte","sox","lei","mig","fig","lon","use","ban","flo","nov","jut","bag","mir","sty","lap","two","ins","con","ant","net","tux","ode","stu","mug","cad","nap","gun","fop","tot","sow","sal","sic","ted","wot","del","imp","cob","way","ann","tan","mci","job","wet","ism","err","him","all","pad","hah","hie","aim"];


fn main() {
    let begin_word = String::from(BEGIN_WORD); // "hit"
    let end_word = String::from(END_WORD); // "cog"
    let  word_list: Vec<String> = WORD_LIST.iter().map(|s| s.to_string()).collect();
    //eprintln!("word list length: {}", word_list.len());
    let ans = Solution::find_ladders(begin_word, end_word, word_list);
    println!("number of solutions: {}", ans.len());
    for sln in ans {
        println!("{:?}", sln);
    }
}

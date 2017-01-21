extern crate graffiti;

use graffiti::corpus::*;
use graffiti::invertedindex::*;

fn main() {

    let mut ii = InvertedIndex::new();

    let brown_corpus = Corpus::brown();

    // get all words from the first 100 docs and save ii
    for d in 0..100 {
            println!("Adding doc {:?}", d);
            ii.add_doc(&brown_corpus.words(d), d as u32);
    }

    // search using some query
    let query: Vec<Vec<u8>> = "dog bark".split(" ").map(|s| s.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    println!("{:?}", ii.get_ranking(&query));


}



extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    // creating string arrays.
    let mut teams_fav: [&str; 10] =  ["Brazil",	"France",	"Argentina",	"England",	"Spain",	"Germany",	"Netherlands",	"Belgium",	"Portugal",	"Denmark"];
    let mut peopl_wcd: [&str; 10] =  ["Tane", "Aymeric", "Libby", "Gilly", "Tristan", "Zoe", "Eddie", "Alex", "Paolo", "Ewan"];
    let mut teams_udg: [&str; 10] =  ["Uruguay", "Croatia","Mexico", "Senegal", "Ecuador", "Serbia", "Poland", "USA", "Wales", "Cameroon"];

    // shuffle each string
    peopl_wcd.shuffle(&mut thread_rng());
    teams_fav.shuffle(&mut thread_rng());
    teams_udg.shuffle(&mut thread_rng());

    for (i, prsn) in peopl_wcd.iter().enumerate() {
        println!("{0} has fave: {1} and underdog: {2}", prsn, teams_fav[i], teams_udg[i]);
    }

}
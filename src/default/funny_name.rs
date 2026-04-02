//! Funny name generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random funny name
pub fn name() -> String {
    fetch_locale("funny_name.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

// Fallback data - puns and funny names
const FALLBACK_NAMES: &[&str] = &[
    "Al B. Zien", "Al Coholic", "Al Kaseltzer", "Al Looya", "Amanda B. Recondwith",
    "Amanda Hugenkiss", "Amanda Lynn", "Anita Bath", "Anita Job", "Anita Knapp",
    "Barb Dwyer", "Barry D'Alive", "Barry McCociner", "Ben Dover", "Ben O. Verbich",
    "Bill Ding", "Bill Loney", "Bjorn Free", "Bo D'Lyn", "Bob Frapples",
    "Brooke Trout", "Bud Jet", "Cara Van", "Chris P. Bacon", "Claire Voyant",
    "Cole Kutz", "Curt N. Call", "Dee Kay", "Dee Liver", "Dinah Might",
    "Don Keigh", "Dusty Rhodes", "E. Ville", "Eileen Dover", "Eli Ondefloor",
    "Estelle Hertz", "Faye Kinnit", "Faye Slift", "Faye Tallity", "Ferris Wheeler",
    "Gabe Asher", "Gene Poole", "Ginger Vitis", "Hal Jalikakick", "Hammond Eggs",
    "Hare Brain", "Harry Pitts", "Hein Noon", "Hellen Back", "Hellen Earth",
    "Herbie Hind", "Holly Day", "Hope Ferterbest", "Howie Doohan", "Hugh Jass",
    "Hugh Jorgan", "Hugh Mungus", "I. P. Freely", "I. Yellalot", "Ima Hogg",
    "Iona Ford", "Iona Frisbee", "Ivana Tinkle", "Jack Pott", "Jane Plain",
    "Jed I. Knight", "Jim Nasium", "Jo King", "Joe King", "Joy Rider",
    "Justin Case", "Justin Credible", "Justin Time", "Kandi Apple", "Kat Lick",
    "Kenny Dewitt", "Kenny Penny", "Kenny U. Putt", "Lance Boyle", "Laura Norder",
    "Leigh King", "Lesa War", "Lily Pond", "Lisa Carr", "Lisa Ford",
    "Lou Briccant", "Lou Stooth", "M. T. Glass", "Mabel Syrup", "Manny Kinn",
    "Marsha Mellow", "Mary Achu", "Mary Christmas", "Mel Lo", "Mike Easter",
    "Mike Oxlong", "Mike Rotch", "Milly Graham", "Missy Sippy", "Misty C. Shore",
    "Mona Lott", "Muffin Man", "Neil Down", "Olive Green", "Olive Yew",
    "Ollie Tabooger", "Otto B. Kilt", "Otto B. Kool", "Otto B. Kute", "Otto B. Kind",
    "Paige Turner", "Peanut Buster", "Pearl E. White", "Penny Lane", "Penny Lope",
    "Phil A. Delphia", "Phil O. Dendron", "Phil O. Sopher", "Phil Updegraff",
    "Polly Esther", "Rick O'Shea", "Rita Book", "Rita Story", "Rocco Z. Disburse",
    "Rocky Beach", "Roman Holiday", "Rose Bush", "Rose Gardner", "Russell Ingleaves",
    "Rusty Bedsprings", "Rusty Blades", "Rusty Carr", "Rusty Dorr", "Rusty Fender",
    "Rusty Keys", "Rusty Nails", "Rusty Pipes", "Rusty Steele", "Sally Forth",
    "Sandy Banks", "Sandy Beech", "Sandy Spring", "Sarah Bellum", "Seymour Butts",
    "Shandy Lear", "Sharon F. Doorcell", "Sharon Needles", "Sheila Blige",
    "Sheri Cola", "Skip Roper", "Stan Still", "Stu Pitt", "Sue Case",
    "Sue Flay", "Sue Ridge", "Sue Shi", "Sue Veneer", "Tad Moore",
    "Tara Misu", "Ted E. Baer", "Terry Aki", "Terry Bull", "Terry Dactyl",
    "Tess Tickle", "Tim Burr", "Tom A. Toe", "Tom Morrow", "Tommy Hawk",
    "Tracy Serpentine", "U. P. Freehly", "U. R. Nice", "Vlad Tire", "Wanda Rinn",
    "Warren Piece", "Wendy Day", "Will Power", "Willie Makit", "Willie Wait",
    "X. Benedict", "Xavier Breth", "Xavier High", "Y. Nott", "Yule B. Sari",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }
}

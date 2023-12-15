use std::env;
use std::fs::read_to_string;

// "one","two","three","four","five","six","seven","eight","nine"
fn main() {
    let file = env::args().nth(1).expect("File not provided");
    let file = read_to_string(file).expect("File does not exist");
    let lines = file.lines();
    let nums = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let word_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let total = lines.into_iter()
        .filter(|line| line.contains(nums) || word_nums.iter().filter(|&&num| line.contains(num)).collect::<Vec<_>>().len() > 0)
        .map(|line| {
            let mut f_pass = nums.iter().zip(word_nums.iter())
                .map(|(&c, &s)| {
                    let c_match = line.match_indices(c).collect::<Vec<_>>();
                    let s_match = line.match_indices(s).map(|v|{
                        let mapped = match v.1 {
                            "one"=>"1",
                            "two"=>"2",
                            "three"=>"3",
                            "four"=>"4",
                            "five"=>"5",
                            "six"=>"6",
                            "seven"=>"7",
                            "eight"=>"8",
                            "nine"=>"9",
                            _=>panic!("should not happen")
                        };
                        return (v.0,mapped)
                    }).collect::<Vec<_>>();
                    [c_match, s_match].concat()
                }).flatten().collect::<Vec<_>>();
            f_pass.sort_by(|a, b| a.cmp(b));

            return f_pass;
        }
        ).fold(0, |acc, e| {
        println!("{:?}",e);
        if e.len() == 0 { return acc; }
        let first = e.first().expect("first element not found");
        let second = e.last().expect("last element does not exist");
        // println!("{:?} {:?}",first,second);
        // println!("{}{}",first.1,second.1);
        let res = format!("{}{}", first.1, second.1).parse::<i32>().unwrap();
        return acc + res;
    });

    println!("total {:?}", total)
}

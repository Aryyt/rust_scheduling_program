use interval::Interval;
use core::panic;
use std::cmp::max;
use std::fs::File;
use std::{i32, usize, vec};
use std::io::{BufRead, BufReader};

mod interval;
fn main() {
    let path:String = "src/weightedIntervals.txt".to_owned();
    let intervals: Vec<Interval> = get_intervals_sorted_finishing(path);
    let mut sum = 0;
    for interval in intervals.iter(){
        sum += interval.weight()
    }
    println!("Sum of all weights is {}", sum);
    schedueling_algo(intervals);
}

fn schedueling_algo(intervals: Vec<Interval>) {
    let mut m = vec![0; intervals.len()];
    let mut k = vec![-1; intervals.len()];
    for (index, _) in intervals.iter().enumerate(){
        get_and_save_score(index, &intervals, &mut m, &mut k);
    }

    let mut max_val = -1;
    for (_, val) in m.iter().enumerate(){
        if *val >= max_val{
            max_val = *val
        }
    }
    println!("Max of non overlaping set is {}", max_val);
}

fn get_and_save_score(index: usize, intervals: &Vec<Interval>, m: &mut Vec<i32>, k: &mut Vec<i32>){
    //First Case
    let predecessor = get_predecessor(index, intervals[index].start(), intervals);
    let mut si: i32 = intervals[index].weight();
    if predecessor >= 0{
        si += m[predecessor as usize];
    }
    //Second Case (Max of previous interval)
    let mut ni = 0;
    if index > 0{
        ni = m[index - 1];
    }

    let max_val = max(si, ni);
    m[index] = max_val;
    if max_val == si{
        k[index] = predecessor
    }else{
        k[index] = (index - 1) as i32
    }
}

fn get_predecessor(index: usize, starting_time: i32, intervals: &Vec<Interval>) -> i32 {
    let mut closest_non_overlaping:i32 = -1;
    for i in (0usize..index).rev() {
        if intervals[i].finish() <= starting_time{
            closest_non_overlaping = i as i32;
            return closest_non_overlaping
        }
    }
    return closest_non_overlaping;
}

fn get_intervals_sorted_finishing(path: String) -> Vec<Interval> {
    // Open the file
    let file = File::open(path);
    let processed = match file {
        Ok(v) => v,
        Err(_) => panic!("Error in reading file"),
      };

    // Create a buffered reader
    let reader = BufReader::new(processed);

    // Iterate over each line
    let mut intervals:Vec<Interval> = Vec::new();
    for line in reader.lines() {
        // Handle potential errors reading each line
        let maybe_line = line;
        let line = match maybe_line {
            Ok(v) => v,
            Err(_) => panic!("Error in reading line"),
          };
        // Process the line (here we just print it)
        let temp = line.split(" ");
        let mut nums: [i32; 3] = [0;3];
        let mut i = 0;
        for number in temp {
            nums[i] = number.parse::<i32>().unwrap();
            i += 1;
//            print!("{} ", nums[i]);
        }
        let inval: Interval = Interval::new(nums[0], nums[1], nums[2]);
        intervals.push(inval);

//        println!("{}", line);
    }
    intervals.sort_by(|a, b| a.finish().cmp(&b.finish()));
    for item in &intervals {
        println!("Finish: {} | Start: {} | Weight: {}", item.finish(), item.start(), item.weight());
    }
    return intervals;
}


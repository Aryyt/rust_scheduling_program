use interval::Interval;
use core::{panic};
use std::collections::btree_map::Range;
use std::fs::File;
use std::ops::Index;
use std::{i32, path, string, usize};
use std::io::{BufRead, BufReader};

mod interval;
fn main() {
    let path:String = "src/weightedIntervals.txt".to_owned();
    let mut intervals: Vec<Interval> = get_intervals_sorted_finishing(path);
    let temp = get_predecessor(0, intervals[0].start(), &intervals);
    println!("Hello, world! hope this works {}", temp);
}

fn schedueling_algo(intervals: Vec<Interval>) {

    for (index, Interval) in intervals.iter().enumerate(){

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


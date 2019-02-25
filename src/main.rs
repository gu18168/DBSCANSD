extern crate dbscansd;

use std::env;
use dbscansd::{
    file_io
};

/// # Should Know
/// * [parse()](https://doc.rust-lang.org/std/primitive.str.html#method.parse))
/// * [expect()](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)
/// 
/// # Example
/// ``` rust
/// execute_dbscansd("input.csv", "output.csv", 20000, 0.03, 50, 2, 2.5, false);
/// ```
fn main() {
    // 第零个参数将会是 target ，应该忽略
    let args: Vec<String>= env::args().collect();

    if args.len() == 8 {
        // parse 用于将 string 转化为其他类型
        // 其中其他类型需要实现 FromStr trait
        // 得到的结果是 Result ，利用 expect 提取取结果以及报错
        let line_num: u32 = args[3].parse().expect("lineNum isn't a Number!");
        let eps: f64 = args[4].parse().expect("eps isn't a Double!");
        let min_pts: u32 = args[5].parse().expect("minPts isn't a Number!");
        let max_spd: f64 = args[6].parse().expect("maxSpd isn't a Double!");
        let max_dir: f64 = args[7].parse().expect("maxDir isn't a Double!");
        let is_stop_point: bool = args[8].parse().expect("isStopPoint isn't a Bool!");
        
        execute_dbscansd(&args[1], &args[2], line_num, eps, min_pts, max_spd, max_dir, is_stop_point);
    } else {
        println!("Please run the program with 8 input parameters: ");
        println!("  args[1]: the input file path");
        println!("  args[2]: the output file path");
        println!("  args[3]: the designated number of trajectory points for clustering");
        println!("  args[4]: eps       - 1st parameter of DBSCANSD, the radius");
        println!("  args[5]: minPoints - 2nd parameter of DBSCANSD, the minimum number of points");
        println!("  args[6]: maxSpd	 - 3rd parameter of DBSCANSD, the maximum Speeds' difference");
        println!("  args[7]: maxDir	 - 4th parameter of DBSCANSD, the maximum Directions' difference");
        println!("  args[7]: boolean value, if you would like to cluster stopping points (true) or moving points (false)");
        println!("e.g. cargo run input.csv output.csv 20000 0.03 50 2 2.5 false");
    }

    // println!("{:?}", args);
}

fn execute_dbscansd(
    in_path: &str, 
    out_path: &str, 
    line_num: u32, 
    eps: f64,
    min_pts: u32,
    max_spd: f64,
    max_dir: f64,
    is_stop_point: bool)
{

}
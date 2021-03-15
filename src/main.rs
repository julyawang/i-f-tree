
use std::fs::File;
use num_traits::pow;

use std::io::Write;
extern crate rand;

use rand::Rng;

fn main() {
//    height_main(6);
    height_main(7);
//    height_main(8);
//    height_main(9);
//    height_main(10);
//    height_main(11);
//    height_main(12);
//    height_main(13);
//    height_main(14);
//    height_main(15);
//    height_main(16);

}

fn width_main(max: usize) {
    let mut file_name: String = "ifw".to_string();
    let mut var: String = "a".to_string();
    let mut rng = rand::thread_rng();
    let min = pow(2u64, max - 1) as u64;
    let max = pow(2u64, max.clone()) as u64;
    let mut random = rng.gen_range::<u64>(min, max);
    for i in 0..101 {
        file_name = format!("ifw{}.zok", i);
//        println!("{}", file_name);
        let mut _file = File::create(file_name.clone()).expect("create failed");
        let mut content = "def main(private field a0".to_string();
        let mut body = format!("    field res = (if a0 >= {} then 1 else 0 fi)", random);
        for j in 1..i {
            var = format!("a{}",j);
//            println!("{}", var);
            content = format!("{}, {}", content, format!("private field {}", var));
            let mut rng = rand::thread_rng();
            let mut random = rng.gen_range::<u64>(min, max);
            body = format!("{} + (if {} >= {} then 1 else 0 fi)", body, var, random);
        }
        content = format!("{}) -> (field): ", content);
        let full = format!("{}\n{}\n    return res", content, body);
        _file.write_all(full.as_bytes()).expect("write failed");
    }

}
fn height_main(max: usize) {
    let mut file_name: String = "ifh".to_string();
    let mut var: String = "a".to_string();
    let mut rng = rand::thread_rng();
    let min = pow(2u64, max - 1) as u64;
    let max = pow(2u64, max.clone()) as u64;
    let mut random = rng.gen_range::<u64>(min, max);
    for i in [1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100].iter() {
        file_name = format!("ifh{}.zok", i);
        let mut _file = File::create(file_name.clone()).expect("create failed");
        let mut content = "def main(private field a0".to_string();
        let mut body = format!("    field res = (if a0 >= {} then ", random);
        for j in 1..*i {
            var = format!("a{}", j);
            content = format!("{}, {}", content, format!("private field {}", var));
            let random = rng.gen_range(min, max);
            body = format!("{}(if {} < {} then ", body, var, random);
            if j == i - 1 {
                body = format!("{} 1 else 0 fi)", body);
                for k in 0..j {
                    body = format!("{} else 0 fi)", body);
                }
            }
        }
        body = format!("{}\n    field verify = if res == a{} then 1 else 0 fi", body, i + 1);
        body = format!("{}\n    return verify", body);
        content = format!("{}, field a{}) -> (field): ", content, i + 1);
        let full = format!("{}\n{}\n", content, body);
        _file.write_all(full.as_bytes()).expect("write failed");
    }
}



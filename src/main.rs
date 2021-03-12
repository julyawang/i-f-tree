
use std::fs::File;


use std::io::Write;
extern crate rand;

use rand::Rng;

fn main() {
    width_main(100);
    height_main(100);
}

fn width_main(max: u64) {
    let mut file_name: String = "ifw".to_string();
    let mut var: String = "a".to_string();
    for i in 0..101 {
        file_name = format!("ifw{}.zok", i);
//        println!("{}", file_name);
        let mut _file = File::create(file_name.clone()).expect("create failed");
        let mut content = "def main(private field a0".to_string();
        let mut body = "    field res = (if a0 >= 20 then 1 else 0 fi)".to_string();
        for j in 1..i {
            var = format!("a{}",j);
//            println!("{}", var);
            content = format!("{}, {}", content, format!("private field {}", var));
            let mut rng = rand::thread_rng();
            let random = rng.gen_range(0, max);
            body = format!("{} + (if {} >= {} then 1 else 0 fi)", body, var, random);
        }
        content = format!("{}) -> (field): ", content);
        let full = format!("{}\n{}\n    return res", content, body);
        _file.write_all(full.as_bytes()).expect("write failed");
    }

}

fn height_main(max: u64) {
    let mut file_name: String = "ifh".to_string();
    let mut var: String = "a".to_string();
    let mut rng = rand::thread_rng();
    for i in 0..101 {
        file_name = format!("ifh{}.zok", i);
        let mut _file = File::create(file_name.clone()).expect("create failed");
        let mut content = "def main(private field a0".to_string();
        let mut body = "    field res = (if a >= 18 then ".to_string();
        for j in 1..i {
            var = format!("a{}",j);
            content = format!("{}, {}", content, format!("private field {}", var));
            let random = rng.gen_range(0, max);
            body = format!("{}(if {} < {} then ", body, var, random);
            if j == i-1 {
                body = format!("{} 1 else 0 fi)", body);
                for k in 0..j {
                    body = format!("{} else 0 fi)", body);
                }
            }
        }
        body = format!("{}\n    field verify = if res == a{} then 1 else 0 fi", body, i+1);
        body = format!("{}\n    return verify", body);
        content = format!("{}, field a{}) -> (field): ", content, i+1);
        let full = format!("{}\n{}\n", content, body);
        _file.write_all(full.as_bytes()).expect("write failed");
    }

}


use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Please input a number");
    let n = get_int() as usize;

    let mut arr = vec![0; n];
    for i in arr.iter_mut() {
        *i = get_int();
    }

    sort(&mut arr[..]);

    for i in arr.iter() {
        print!("{}, ", i);
    }
    println!("");
}

fn get_int_rst() -> Result<i32, ParseIntError> {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    let ret: i32 = ret.trim().parse()?;
    Ok(ret)
}

fn get_int() -> i32 {
    loop {
        match get_int_rst() {
            Ok(ret) => return ret,
            Err(_) => println!("Please input a int"),
        }
    }
}

fn sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let std = arr[len - 1];
    let mut less = 0;
    for i in 0..len {
        if arr[i] <= std {
            arr.swap(i, less);
            less += 1;
        }
    }

    sort(&mut arr[..less - 1]);
    sort(&mut arr[less..]);
}

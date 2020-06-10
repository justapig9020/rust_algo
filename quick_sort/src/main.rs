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
    println!("len: {}; std: {}", len, std);
    let mut less = 0;
    for i in 0..len {
        if arr[i] <= std {
            swap(&mut arr[less..less], &mut arr[i..i]);
            /*
            arr[less] ^= arr[i];
            arr[i] ^= arr[less];
            arr[less] ^= arr[i];
            */

            println!("{} @ {}", less, arr[less]);
            println!("{} @ {}", i, arr[i]);
            less += 1;
        }
    }

    println!("");
    for i in arr.iter() {
        print!("{}, ", i);
    }
    println!("");

    sort(&mut arr[..less - 1]);
    sort(&mut arr[less..]);
}

fn swap(a: &mut [i32], b: &mut [i32]) {
    a[0] ^= b[0];
    b[0] ^= a[0];
    a[0] ^= b[0];
}

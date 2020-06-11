use std::io;
use std::num::ParseIntError;

fn main() {
    let n = get_int();
    let mut arr = vec![0; n as usize];

    for i in arr.iter_mut() {
        *i = get_int();
    }

    sort(&mut arr);

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

    let mid = len / 2;
    let mut sub1 = arr[..mid].to_vec();
    let mut sub2 = arr[mid..].to_vec();

    sort(&mut sub1);
    sort(&mut sub2);

    let mut i = 0;
    while !sub1.is_empty() && !sub2.is_empty() {
        arr[i] = if sub1[0] > sub2[0] {
            sub2.remove(0)
        } else {
            sub1.remove(0)
        };
        i += 1;
    }
    while !sub1.is_empty() {
        arr[i] = sub1.remove(0);
        i += 1;
    }
    while !sub2.is_empty() {
        arr[i] = sub2.remove(0);
        i += 1;
    }
}

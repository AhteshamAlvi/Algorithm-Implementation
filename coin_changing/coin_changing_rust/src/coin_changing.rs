use std::cmp::min;
use std::i32::MAX;

fn coin_changing(c: &Vec<i32>, n: i32)-> Vec<i32> {
    let mut a = vec![i32::MAX; n as usize];
    a[0] = 0;
    let mut coin_num: i32;

    for i in 1..n {
        coin_num = MAX;
        for &coin in c {
            if i - coin >= 0 {
                coin_num = min(coin_num, 1 + a[(i - coin) as usize]);
            }
        }
        a[i as usize] = coin_num;
    }
    return a
}

fn main() {
    let b = vec![1,5,10,25];
    let d = coin_changing(&b, 1434);
    println!("{}", d[1023])
}

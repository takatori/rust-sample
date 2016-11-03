
fn sum_vec(memo: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in memo.iter() {
        sum = sum + i;
    }
    sum
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    sum_vec(&v1) + sum_vec(&v2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer =  foo(v1, v2);
    println!("{}", answer);
}

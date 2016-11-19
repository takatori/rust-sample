// `NanoSecond` を `u64`の別名として使用する。
type NanoSecond = u64;
type Inch = u64;

// 警告を抑えるアトリビュートを使用
#[allow(non_camel_case_types)]
type u64_t = u64;

#[allow(trivial_numeric_casts)]
fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    // 型のエイリアスは、元の型をより型安全にしてくれる**わけではない**ことに注意
    // なぜならば、エイリアスは新たな型を定義している**わけではない**から
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

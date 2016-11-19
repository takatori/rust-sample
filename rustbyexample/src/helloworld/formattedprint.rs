///
///
/// ``format!`` : Stringにフォーマットされたテキストを出力する
/// ``print!``  : format!と同じだが、コンソールに出力する
/// ``println!``: print!と同じだが、改行する
fn main() {
    
    println!("{} days", 31);

    // 位置指定できる
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 名前付き引数もできる
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // ':'に続く文字で特別なフォーマットができる。 :b -> binary
    // @see https://doc.rust-lang.org/beta/std/fmt/
    println!("{} of {:b} know binary, the other half don't", 1, 2);

    // 横幅を指定して右詰めできる
    println!("{number:>width$}", number=1, width=6);

    // ゼロパディング
    println!("{number:0width$}", number=1, width=6);

    // `i32`を保持する`Structure`という名の構造体を定義します
    struct Structure(i32);
    
    // この様にカスタム型を用いる場合、扱いが少々複雑になります。
    // 以下は動作しません。
    println!("This struct `{}` wont't print...", Structure(3));
}

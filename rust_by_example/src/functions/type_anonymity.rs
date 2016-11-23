// `F`は`Fn`を実装していなくてはならず、`Fn`は引数と返り値を持たない。
// `print`は文字列をプリントするだけのクロージャなので、これが正しい
fn apply<F>(f: F) where
    F: Fn() {
    f()
}


fn main() {
    let x = 7;

    // `x`を無名の構造体に入れ、それに対し`Fn`を実装する。
    // ここでは`Fn`は`fn Fn(&self) -> { println!("{}", &self) }`
    let print = || println!("{}", x);

    apply(print);
}

fn main() {
    // `Option<i32>`の`optional`を作成
    let mut optional = Some(0);

    // これはつぎのように読める。「`let`が`optional`を`Some(i)`にデストラクトしている間は
    // ブロック内(`{}`)を評価せよ。さもなくば`break`せよ。」
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

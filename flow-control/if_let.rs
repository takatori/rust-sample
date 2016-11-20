fn main() {
    // すべて`Option<i32>`型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let`文は以下と同じ意味
    //
    // もしletがnumberをデストラクトした結果が`Some(i)`になるならば
    // ブロック内(`{}`)を実行する。
    if let Some(i) = number {
        println!("Matched {:?}!", i)
    }

    // デストラクトした結果が`Some()`にならない場合の処理を明示したい場合、
    // `else`を使用する
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // デストラクトの失敗。このブロック内を実行
        println!("Didn't match a number. Let's go with a letter! ");
    };

    // デストラクト失敗時の処理を更に分岐させることもできる
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // デストラクト失敗時。`else if`を評価し、処理をさらに分岐させる。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

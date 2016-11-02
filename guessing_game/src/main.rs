extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);
    
    loop {
        
        println!("Please input your guess.");
        
        // 変数束縛を作るlet文
        // EX: let foo = bar;
        // デフォルトでイミュータブル
        // mutを使うとミュータブルになる
        //
        // Stringは文字列型
        // 伸長可能でUTF-8でエンコードされたテキスト片
        //
        // ::new() -> 特定の型の「関連関数」
        // StringインスタンスではなくString自体に関連付けられている(スタティックメソッド)
        // 新たな空のStringを作る
        let mut guess = String::new();

        // io::stdin() -> 関連関数(スタティックメソッド)呼び出し。標準入力へのハンドルを返す
        // メソッド -> 型自体ではなくインスタンスに対してだけ使える
        // .read_line(&mut guess) -> ユーザからの入力を取得
        // 参照を受け取る
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // シャドーイング
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("Yow win!");
                break;
            }      
        }
    }
}

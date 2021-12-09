use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    // Mutexは並行処理を制御するための機構
    // 内容へ同時アクセスできるのは1スレッドに限定される
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    // selfを取らないため関連関数
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    // メソッドは明示的なselfパラメータを取る
    fn eat(&self, table: &Table) {
        // 変数を使用しない場合はアンダースコアをつけることで警告を出さないようにできる
        // _leftと_rightがスコープから抜けるときに自動的にロックが解放される
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));
        
        println!("{} is done eating.", self.name);
    }
}

fn main() {

    // arc -> アトミック参照カウント
    // 共有するときは参照カウントを増やし、各スレッドの終了時はカウントを減らす
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),        
    ]});
        
    // Vec<T> -> 可変長の配列型
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),        
    ];

    // スレッドはそのスレッドを制御するハンドルを返す
    // into_iter -> イテレータを生成する
    // throw::spawn -> クロージャを一つ引数にとり、新しいスレッド上でそのクロージャを実行する
    // 特別なアノテーションmoveを必要とする
    // キャプチャする値の所有権がクロージャ内へ移動する
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();
        
    for h in handles {
        // ハンドルへのjoin()呼び出しをループし、各スレッド実行が完了するまで実行をブロックする
        h.join().unwrap();
    }

}

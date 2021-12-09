use List::*;

enum List {
    // Cons: これは、要素をラップし、つぎの要素へのポインタを保持するタプル。
    Cons(u32, Box<List>),
    // Nil: 連結リストの終端であることを示すノード
    Nil,
}

// 列挙型にはメソッドを付与することができる。
impl List {
    // カラリストの作成
    fn new() -> List {
        // `Nil`は`List`型を持つ。
        Nil
    }

    // リストを受取り、その始端に新しい要素を付加したものを返す関数
    fn prepend(self, elem: u32) -> List {
        // この`Cons`自体も、その第2要素もどちらもlist型である。
        Cons(elem, Box::new(self))
    }

    // listの長さを返すメソッド
    fn len(&self) -> u32 {
        // このメソッドは、`self`の状態によって振る舞いが変化するため
        // matchをする必要がある。
        // `self`の型は`&List`であるので、`*self`は`List`になるマッチングは
        // リファレンス( `&T` )でなく実態( `T` )に対して行うのが好ましい
        match *self {
            // `self`をすでに借用しているので、tailの所有権を取ることができない。
            // 代わりに参照を使用する。
            Cons(_, ref tail) => 1 + tail.len(),
            // 空リストならば長さは0
            Nil => 0
        }
    }

    // Listをheap上の文字列として表したものを返すメソッド。
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!`は`print!`に似ているが、コンソールに出力する代わりに
                // heap上の文字列を返す。
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 空の連結リストを作成
    let mut list = List::new();

    // 要素を追加
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 追加後の状態を表示
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());    
}

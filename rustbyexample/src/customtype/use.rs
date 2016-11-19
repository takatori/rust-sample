// 使用されていないコードによる警告を隠すアトリビュート
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Solider,
}

fn main() {
    // `use`することで絶対名でなくても使用可能になる。
    use Status::{Poor, Rich};
    // `Work`の中の名前をすべて`use`する
    use Work::*;

    // `use`しているため、`Status::Poor`と書いていることに等しい
    let status = Poor;
    // `Work::Civilian`に等しい
    let work = Civilian;

    match status {
        // `use`しているのでスコープを明示していない
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),        
    }

    match work {
        Civilian => println!("Civilians work!"),
        Solider => println!("Soliders fight!"),        
    }
}

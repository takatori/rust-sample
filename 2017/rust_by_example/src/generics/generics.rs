// `A`という具象型
struct A;

// `Single`という型を定義するために`A`を使用しているが、その最初の仕様よりも
// 先に`<A>`がないため、また、`A`自身も具象型であるため、`Single`は具象型となる。
struct Single(A);

// ここでは`<T>`が一番初めの`T`の仕様よりも先に来ている。よって`SingleGen`はジェネリック型となる
// なぜならば型パラメータ`T`がジェネリックだからである。`T`はどんな方にもなり得るため、
// 上で定義した`A`を受け取ることができる。
struct SingleGen<T>(T);

fn main() {
    // `Single`は具象型で`A`のみを受け取る
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t    = SingleGen(A);
    let _i32  = SingleGen(6);
    let _char = SingleGen('a');
}

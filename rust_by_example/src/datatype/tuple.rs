use std::fmt;

// タプルを関数の引数及び返り値として使用している。
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `letでタプルの中の値を別の変数に束縛することができる`
    let (integer, boolean) = pair;

    (boolean, integer)
}
    
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


// 演習
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {

    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    // 様々な型を値に持つタプル
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64, 'a', true);

    // インデックスを用いて、タプル内の要素を参照できる。
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // タプルはタプルのメンバになれる。
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // タプルはプリント可能
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("par is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 要素を一つしか持たないタプルを作成する場合、
    // カッコで囲まれたただのリテラルと区別するため、カンマが必要になる。
    println!("one element tuple: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    // タプルを分解して別の変数にそれぞれの値を代入
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

        
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));    
}

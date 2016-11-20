use std::fmt; // Import `fmt`

// 2つの数字を扱うための構造体です。出力を`Display`と比較するため
// `Debug`をDeriveしています
#[derive(Debug)]
struct MinMax(i64, i64);

// `MinMax`用の`Display`を実装しています。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 比較のため、フィールドに名前をつけれるような構造体を定義
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// 先程と同様にして、Point2用の`Display`を実装しています。
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `x`と`y`のみが明示的になるようにカスタマイズ
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare pionts:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // `Debug`と`Display`は実装されていますが、`fmt::Binary`はされていないため
    // `{:b}`を使用している以下の例はエラーになります
    // println!("What does Point2D look like in binary: {:b}?", point)


    // 演習
    let complex = Complex { real: 1.2, imag: 3.2 };
    
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);    
}

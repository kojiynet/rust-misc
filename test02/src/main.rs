
/*

test02/main.rs

cargoでつくるとき：
　cargo new XXX
で、プロジェクトのディレクトリXXXがつくられる。
XXXの中のsrcディレクトリにソースファイルを置く。
XXXの中のCargo.tomlにある[dependencies]以下に、追加で必要なcrateを記述。
リリースビルドにするには、ディレクトリ内で
　cargo build --release

*/

use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;

fn main()
{

    println!("p value; CDF(0.975) for N(0,1)");

    let n = Normal::new(0.0, 1.0).unwrap();
    let p = n.inverse_cdf( 0.975);

    println!("p = {p}");

}

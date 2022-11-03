
/*

test04/main.rs

test02を改造する。
Beta分布のCDFの逆関数を出してみる。

Beta( 0.5, 0.5)のInvCDF(0.01)は、
Rustだと
　0.000274658203125
Rだと
　> qbeta( 0.01, 0.5, 0.5)
　[1] 0.0002467198
Excelだと
　=BETA.INV(0.01,0.5,0.5)
　0.00024672

→Rustの精度が低い？


cargoでつくるとき：
　cargo new XXX
で、プロジェクトのディレクトリXXXがつくられる。
XXXの中のsrcディレクトリにソースファイルを置く。
XXXの中のCargo.tomlにある[dependencies]以下に、追加で必要なcrateを記述。
リリースビルドにするには、ディレクトリ内で
　cargo build --release

*/

use statrs::distribution::Beta;
use statrs::distribution::ContinuousCDF;
// use statrs::distribution::Normal;

fn main()
{

    println!("z = InvCDF(p) for Beta(alpha,beta)");

    let alpha = 0.5;
    let beta = 0.5;
    let dist = Beta::new( alpha, beta).unwrap();

    println!( "alpha\tbeta\tp\tz");
    for pby100 in 1..=99 {
        let p = ( pby100 as f64) / 100.0;
        let z = dist.inverse_cdf( p);
        println!( "{alpha}\t{beta}\t{p}\t{z}");
    }

}

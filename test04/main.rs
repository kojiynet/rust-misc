
/*

test04/src/main.rs

test02を改造する。
Beta分布のCDFの逆関数を出してみる。

Beta( 0.5, 0.5)のInvCDF(0.01)は、
Rustだと
　0.000274658203125
C++ (Boost)だと
　0.00024672
Rだと
　> qbeta( 0.01, 0.5, 0.5)
　[1] 0.0002467198
Excelだと
　=BETA.INV(0.01,0.5,0.5)
　0.00024672

→RustのCDFに入れなおしても誤差がある。

→RustのInverseCDFの精度が低い？


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

    println!( "alpha\tbeta\tp\tz\tcalculated_p\tabserr");

    for a_int in 1..=4 {
        for b_int in 1..=4 {

            let alpha = ( a_int as f64) * 0.5;
            let beta = ( b_int as f64) * 0.5;
            let dist = Beta::new( alpha, beta).unwrap();
            for pby100 in 1..=99 {
                let p = ( pby100 as f64) / 100.0;
                let z = dist.inverse_cdf( p);
                let calculated_p = dist.cdf( z);
                let abserr = ( p - calculated_p).abs();
                println!( "{alpha}\t{beta}\t{p}\t{z}\t{calculated_p}\t{abserr}");
            }

        }
    }

}

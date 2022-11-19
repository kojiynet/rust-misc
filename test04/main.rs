
/*

test04/src/main.rs

Beta分布のCDFを出してみる。

Beta( 0.5, 0.5)のCDF(0.01)は、

Rust (crate: statr) だと
　0.06376856085851985
Rust (crate: probability) だと
　0.06376856085851985

→その他の場合も、Rustのstatrとprobabilityはほぼ等しい。

cargoでつくるとき：
　cargo new XXX
で、プロジェクトのディレクトリXXXがつくられる。
XXXの中のsrcディレクトリにソースファイルを置く。
XXXの中のCargo.tomlにある[dependencies]以下に、追加で必要なcrateを記述。
リリースビルドにするには、ディレクトリ内で
　cargo build --release

*/

fn main()
{

    let alpha_array = [ 0.5, 1.0, 1.5, 2.0];
    let beta_array = alpha_array;
    
    println!( "Test for CDF() of Beta(alpha,beta)");
    println!( " p_hat = CDF(z) for various z values");
    println!();
    println!( "Test two crates; \"1\" means \"statrs\" crate, \"2\" means \"probability\" crate");
    println!();

    println!( "alpha\tbeta\tz\tp_hat_1\tp_hat_2");

    for alpha in alpha_array {
        for beta in beta_array {
            for zby100 in 1..=99 {
                
                let z = ( zby100 as f64) / 100.0;

                let p_hat_1 = beta_cdf_statrs_crate( alpha, beta, z);
                let p_hat_2 = beta_cdf_prob_crate( alpha, beta, z);

                println!( "{alpha}\t{beta}\t{z}\t{p_hat_1}\t{p_hat_2}");

            }
        }
    }

}

fn beta_cdf_statrs_crate( alpha: f64, beta: f64, z: f64) -> f64 
{

    use statrs::distribution::Beta;
    use statrs::distribution::ContinuousCDF;

    let dist = Beta::new( alpha, beta).unwrap();
    let p_hat = dist.cdf( z);

    return p_hat;

}

fn beta_cdf_prob_crate( alpha: f64, beta: f64, z: f64) -> f64
{

    use probability::distribution::Beta;
    use probability::distribution::Distribution;

    let dist = Beta::new( alpha, beta, 0.0, 1.0);
    let p_hat = dist.distribution( z);

    return p_hat;

}

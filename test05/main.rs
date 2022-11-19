
/*

test05/src/main.rs

Beta分布のCDFの逆関数を出してみる。

このプログラムでのpとp_hatの絶対誤差の最大値
　crate: statr
　　0.00218257662251974
　crate: probability
　　2.44249065417534E-15

Beta( 0.5, 0.5)のInvCDF(0.01)は、
Rust (crate: statr) だと
　0.000274658203125
Rust (crate: probability) だと
　0.00024671981713422173
C++ (Boost)だと
　0.00024672
Rだと
　> qbeta( 0.01, 0.5, 0.5)
　[1] 0.0002467198
Excelだと
　=BETA.INV(0.01,0.5,0.5)
　0.00024672

→Rustのstatrだと、InvCDF()の精度が低そう。

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
    
    println!( "Test for InvCDF() of Beta(alpha,beta)");
    println!( " Assume p = CDF(z)");
    println!( " First, set p, and then calculate z_hat using InvCDF()");
    println!( " After that, calculate p_hat = CDF(z_hat) for validation");
    println!();
    println!( "Test two crates; \"1\" means \"statrs\" crate, \"2\" means \"probability\" crate");
    println!();

    println!( "alpha\tbeta\tp\tz_hat_1\tp_hat_1\tabserr_1\tz_hat_2\tp_hat_2\tabserr_2");

    for alpha in alpha_array {
        for beta in beta_array {
            for pby100 in 1..=99 {
                
                let p = ( pby100 as f64) / 100.0;

                let ( z_hat_1, p_hat_1, abserr_1) = beta_statrs_crate( alpha, beta, p);
                let ( z_hat_2, p_hat_2, abserr_2) = beta_prob_crate( alpha, beta, p);

                println!( "{alpha}\t{beta}\t{p}\t{z_hat_1}\t{p_hat_1}\t{abserr_1}\t{z_hat_2}\t{p_hat_2}\t{abserr_2}");

            }
        }
    }

}

fn beta_statrs_crate( alpha: f64, beta: f64, p: f64) -> ( f64, f64, f64) 
{

    use statrs::distribution::Beta;
    use statrs::distribution::ContinuousCDF;

    let dist = Beta::new( alpha, beta).unwrap();
    let z_hat = dist.inverse_cdf( p);
    let p_hat = dist.cdf( z_hat);
    let abserr = ( p - p_hat).abs();

    return ( z_hat, p_hat, abserr);

}

fn beta_prob_crate( alpha: f64, beta: f64, p: f64) -> ( f64, f64, f64)
{

    use probability::distribution::Beta;
    use probability::distribution::Inverse;
    use probability::distribution::Distribution;

    let dist = Beta::new( alpha, beta, 0.0, 1.0);
    let z_hat = dist.inverse( p);
    let p_hat = dist.distribution( z_hat);
    let abserr = ( p - p_hat).abs();

    return ( z_hat, p_hat, abserr);

}


/*

test06/src/main.rs

Beta分布のCDFの逆関数を出してみる。
C++/Boostなどと比べるために。

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

    let alpha_array = [ 0.5, 1.0, 2.0];
    let beta_array = alpha_array;
    
    println!( "Test for InvCDF() of Beta(alpha,beta)");
    println!( " Set p = {{ 0.01, 0.02, ... , 0.98, 0.99}}");
    println!( " Then calculate z_hat using InvCDF()");
    println!();
    println!( "Test two crates; \"1\" means \"statrs\" crate, \"2\" means \"probability\" crate");
    println!();

    println!( "alpha\tbeta\tp\tz_hat_1\tz_hat_2");

    for alpha in alpha_array {
        for beta in beta_array {
            for pby100 in 1..=99 {
                
                let p = ( pby100 as f64) / 100.0;

                let z_hat_1 = beta_invcdf_statrs_crate( alpha, beta, p);
                let z_hat_2 = beta_invcdf_prob_crate( alpha, beta, p);

                println!( "{alpha}\t{beta}\t{p}\t{z_hat_1}\t{z_hat_2}");

            }
        }
    }

}

fn beta_invcdf_statrs_crate( alpha: f64, beta: f64, p: f64) -> f64
{

    use statrs::distribution::Beta;
    use statrs::distribution::ContinuousCDF;

    let dist = Beta::new( alpha, beta).unwrap();
    let z_hat = dist.inverse_cdf( p);
    
    return z_hat;

}

fn beta_invcdf_prob_crate( alpha: f64, beta: f64, p: f64) -> f64
{

    use probability::distribution::Beta;
    use probability::distribution::Inverse;
    // use probability::distribution::Distribution;

    let dist = Beta::new( alpha, beta, 0.0, 1.0);
    let z_hat = dist.inverse( p);

    return z_hat;

}


/*

test07/src/lib.rs

CargoでWASM
Beta分布のInvCDFをウェブで。。

次のコマンドでプロジェクト／ディレクトリを作成
  cargo new --lib test07

Cargo.tomlに以下を追加

  [lib]
  crate-type = ["cdylib"]

  [dependencies]
  probability = "0.20.3"


※ここまでやった

次のコマンドでデバッグビルド
  cargo build --target=wasm32-unknown-unknown
次のファイルが生成される
  test07\target\wasm32-unknown-unknown\debug\test03.wasm

今回はやらない：次のコマンドでリリースビルド
  cargo build --target=wasm32-unknown-unknown --release
今回はやらない：次のファイルが生成される
  test07\target\wasm32-unknown-unknown\release\test03.wasm

以下のディレクトリにHTMLとWASMを入れた
  test07/_site

*/

// this file does not contain a main function
#![no_main]

// we do not want to mangle the symbol when exporting
#[no_mangle]
pub extern fn beta_invcdf( alpha: f64, beta: f64, p: f64) -> f64
{

    use probability::distribution::Beta;
    use probability::distribution::Inverse;

    let dist = Beta::new( alpha, beta, 0.0, 1.0);
    let z_hat = dist.inverse( p);

    return z_hat;

}


/*

test03/src/lib.rs

CargoでWASM

次のコマンドでプロジェクト／ディレクトリを作成
  cargo new --lib test03
Cargo.tomlに以下を追加
  [lib]
  crate-type = ["cdylib"]
次のコマンドでデバッグビルド
  cargo build --target=wasm32-unknown-unknown
次のファイルが生成される
  test03\target\wasm32-unknown-unknown\debug\test03.wasm
次のコマンドでリリースビルド
  cargo build --target=wasm32-unknown-unknown --release
次のファイルが生成される
  test03\target\wasm32-unknown-unknown\release\test03.wasm

以下のディレクトリにHTMLとWASMを入れた
  test03/_site

*/

#![no_main]    // this file does not contain a main function

#[no_mangle]   // we do not want to mangle the symbol when exporting
pub extern fn add(a: i32, b: i32) -> i32 { a + b }

// mod.rs という名前のファイルがあるとそのディレクトリをモジュールと認識する
// モジュールを宣言して、実装は別のファイル
mod r#box;
mod box_spot;
mod floor;
mod label;
mod player;
mod wall;

// pub use で再公開する(外部のモジュールに対して)
pub use self::{box_spot::*, floor::*, label::*, label::*, player::*, r#box::*, wall::*};

// GameEventを公開する(enumなので中身も公開)
pub enum GameEvent {
    // プレイヤーが障害物(例えば壁)に当たった時に実行される
    PlayerHitObstacle,

    // エンティティが動いた時に実行される
    EntityMoved(EntityId),

    // // 箱がスポットに置かれた or 置かれてないの時に実行される
    // BoxPlacedOnSpot(IsCorrectSpot),
}

// #[derive(Debug)] で fmt::Debug でプリントするための実装を提供する
#[derive(Debug)]
pub struct EntityId(pub u32);

// #[derive(Debug)]
// pub struct IsCorrectSpot(pub bool);
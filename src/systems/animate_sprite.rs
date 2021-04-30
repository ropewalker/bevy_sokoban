use bevy::prelude::*;

// スプライトアニメーションをtime分更新する
pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
){
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut(){
        // 時間を進ませる
        timer.tick(time.delta());
        // 時間が経過すれば、アトラスから次のIndexを設定する
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
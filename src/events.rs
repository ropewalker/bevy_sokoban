pub enum GameEvent {
    // Fired when the player hits an obstacle like a wall
    PlayerHitObstacle,

    // Fired when an entity is moved
    EntityMoved(EntityId),
}

#[derive(Debug)]
pub struct EntityId(pub u32);

#[derive(Debug)]
pub struct IsCorrectSpot(pub bool);

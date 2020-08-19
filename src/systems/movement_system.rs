use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

pub fn movement_system(
    mut events: ResMut<Events<GameEvent>>,
    entity: Entity,
    mut movable: Mut<Movable>,
    mut position: Mut<Position>,
) {
    use crate::components::MoveDirection::*;

    if let Some(direction) = movable.direction.take() {
        match direction {
            Up => position.y -= 1,
            Down => position.y += 1,
            Left => position.x -= 1,
            Right => position.x += 1,
        }

        events.send(GameEvent::EntityMoved(EntityId(entity.id())));
    }
}

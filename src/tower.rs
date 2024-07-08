use bevy::prelude::*;
use strum::EnumIter;

use crate::{
    states::GameSystemSet,
    typing::{Action, AddToType},
};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<TowerType>()
            .add_systems(Update, insert_tower_typing.in_set(GameSystemSet))
            .observe(change_tower_priority);
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub priority: TowerPriority,
}
impl Tower {
    pub const fn new(tower_type: TowerType, priority: TowerPriority) -> Self {
        Self {
            tower_type,
            priority,
        }
    }
}

#[derive(Reflect, Clone, Copy, Debug)]
pub enum TowerType {
    Fire,
    Arrow,
}

#[derive(Reflect, Clone, Copy, Debug, EnumIter)]
pub enum TowerPriority {
    /// The enemy that is the nearest to the tower
    Nearest,
    /// The enemy that is the furthest on the path
    Furthest,
}

fn insert_tower_typing(
    towers: Query<(Entity, Option<&Children>), With<Tower>>,
    mut commands: Commands,
) {
    for (entity, children) in &towers {
        if children.map_or(true, |children| children.is_empty()) {
            commands.trigger_targets(AddToType(Action::SpawnArrow(entity), None), entity);
        }
    }
}

#[derive(Debug, Clone, Event)]
pub struct ChangeTowerPriority(pub TowerPriority);

pub fn change_tower_priority(trigger: Trigger<ChangeTowerPriority>, mut towers: Query<&mut Tower>) {
    let mut tower = towers
        .get_mut(trigger.entity())
        .expect("Entity use to trigger this function should be in query");
    tower.priority = trigger.event().0;
}

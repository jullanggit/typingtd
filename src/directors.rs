use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{enemy::Enemy, oneshot::OneShotSystems, physics::apply_position};

pub struct DirectorPlugin;
impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Difficulty>()
            .init_resource::<Director>()
            .register_type::<Director>()
            .register_type::<Difficulty>()
            .add_systems(Update, update_director)
            .add_systems(FixedUpdate, spawn_enemies.after(apply_position));
    }
}

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
struct Director {
    credits: f64,
    credit_rate: f64,
}
impl Default for Director {
    fn default() -> Self {
        Self {
            credits: 0.0,
            credit_rate: 1.0,
        }
    }
}

#[derive(Resource, Debug, Clone, Reflect, Default)]
#[reflect(Resource)]
enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}
impl Difficulty {
    const fn multiplier(&self) -> f64 {
        match self {
            Self::Easy => 0.5,
            Self::Normal => 1.0,
            Self::Hard => 2.0,
        }
    }
}

fn update_director(mut director: ResMut<Director>, difficulty: Res<Difficulty>, time: Res<Time>) {
    director.credit_rate += 0.05 * difficulty.multiplier() * time.delta_seconds_f64();
    director.credits += director.credit_rate * difficulty.multiplier() * time.delta_seconds_f64();
}

fn spawn_enemies(
    mut director: ResMut<Director>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    if thread_rng().gen_range(0..100) == 69 {
        let random_enemy = Enemy::random();
        let random_enemy_cost = random_enemy.cost();
        if random_enemy_cost <= director.credits {
            director.credits -= random_enemy_cost;

            commands.run_system_with_input(oneshot_systems.spawn_enemy, random_enemy);
        }
    }
}

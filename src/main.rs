mod data;
use bevy::prelude::*;
use data::{Field, Cell,};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};


#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app: App = App::new();
    app
        .add_startup_system(camera_setup)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .init_resource::<Field>()
        .add_plugin(FieldPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app.run();
}

#[derive(Component)]
struct World;

#[derive(Resource)]
struct UpdateTimer(Timer);

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpdateTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
            .add_startup_system(init_field)
            .add_system(update_field);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_field(f: ResMut<Field>) {
    f.get_mat()
        .par_iter_mut().for_each(
            |p| p.par_iter_mut().for_each(|c| init_cell(c)));

}

fn init_cell(c: &mut Cell) {
    c.alive = rand::random::<bool>();
}

fn update_field(time: Res<Time>, mut timer: ResMut<UpdateTimer>, f: ResMut<Field>) {
    if timer.0.tick(time.delta()).just_finished() {
        f.get_mat()
        .par_iter_mut().for_each(
            |p| p.par_iter_mut().for_each(|c| update_cell(c, &f)));

    }
}
enum Fold {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    None
}

fn get_fold(f: &Field, x: usize, y: usize) -> Fold {
    let x_max = f.get_width()-1;
    let y_max = f.get_height()-1;

    if x == 0 {
        if y == 0 {
            Fold::NW
        } else if y == y_max{
            Fold::SW
        } else {
            Fold::W
        }
    } else if x == x_max{
        if y == 0 {
            Fold::NE
        } else if y == y_max{
            Fold::SE
        } else {
            Fold::E
        }
    } else {
        if y == 0 {
            Fold::N
        } else if y == y_max{
            Fold::S
        } else {
            Fold::None
        }
    }
}

fn update_cell(b: &mut Cell, f: &Field) {
    let x = b.get_pos().0;
    let y = b.get_pos().1; 
    let fold = get_fold(f, x, y);
    let mut count_surrounding_alive = 0u32;
    match fold {
        Fold::NW => {
            let test_cells: [Cell; 3] = [f.get_mat()[x][y+1], f.get_mat()[x+1][y+1], f.get_mat()[x+1][y]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::N => {
            let test_cells: [Cell; 5] = [f.get_mat()[x-1][y], f.get_mat()[x-1][y+1], f.get_mat()[x][y+1], f.get_mat()[x+1][y+1], f.get_mat()[x+1][y]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::NE => {
            let test_cells: [Cell; 3] = [f.get_mat()[x][y+1], f.get_mat()[x-1][y+1], f.get_mat()[x-1][y]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::E => {
            let test_cells: [Cell; 5] = [f.get_mat()[x][y-1], f.get_mat()[x-1][y-1], f.get_mat()[x-1][y], f.get_mat()[x-1][y+1], f.get_mat()[x][y+1]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::SE => {
            let test_cells: [Cell; 3] = [f.get_mat()[x][y-1], f.get_mat()[x-1][y-1], f.get_mat()[x-1][y]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::S => {
            let test_cells: [Cell; 5] = [f.get_mat()[x-1][y], f.get_mat()[x-1][y-1], f.get_mat()[x][y-1], f.get_mat()[x+1][y-1], f.get_mat()[x+1][y]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::SW => {
            let test_cells: [Cell; 3] = [f.get_mat()[x][y-1], f.get_mat()[x+1][y-1], f.get_mat()[x+1][y]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::W => {
            let test_cells: [Cell; 5] = [f.get_mat()[x][y-1], f.get_mat()[x+1][y-1], f.get_mat()[x+1][y], f.get_mat()[x+1][y+1], f.get_mat()[x][y+1]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        },
        Fold::None => {
            let test_cells: [Cell; 8] = [f.get_mat()[x][y-1], f.get_mat()[x+1][y-1], f.get_mat()[x+1][y], f.get_mat()[x+1][y+1], f.get_mat()[x][y+1], f.get_mat()[x-1][y+1], f.get_mat()[x-1][y], f.get_mat()[x-1][y-1]];
            for i in 0..test_cells.len() {
                if test_cells[i].alive {
                    count_surrounding_alive += 1;
                }
            }
        }
    }

    if f.get_mat()[x][y].alive && 2 <= count_surrounding_alive && count_surrounding_alive <= 3 {
        b.alive = true;
    } else if !f.get_mat()[x][y].alive && count_surrounding_alive == 3 {
        b.alive = true;
    } else {
        b.alive = false;
    }

}
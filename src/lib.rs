#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod acmd;
mod opff;
mod status;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smashline::*;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    let agent = &mut Agent::new("luigi");
    acmd::install(agent);
    opff::install();
    status::install();
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("dash_speed"), 0, 2.4));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("run_speed_max"), 0, 3.3));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("walk_speed_max"), 0, 1.5));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("air_speed_x_stable"), 0, 1.25));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("air_speed_y_stable"), 0, 1.8));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("air_accel_y"), 0, 0.21));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("dive_speed_y"), 0, 2.88));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("jump_y"), 0, 35.0));
    param_config::update_float_2(*FIGHTER_KIND_LUIGI, vec![16].clone(), (hash40("mini_jump_y"), 0, 16.4));
}
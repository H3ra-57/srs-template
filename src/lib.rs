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

pub static mut VOLLEY: i32 = 2;

pub const WEAPON_GANON_VOLLEY_STATUS_KIND_MOVE: LuaConst = LuaConst::new(0x0);

pub const VOLLEY_DAMAGE: LuaConst = LuaConst::new(0x52);
pub const VOLLEY_SCALE: LuaConst = LuaConst::new(0x53);

pub const VOLLEY_EFFECT: LuaConst = LuaConst::new(0x100000C4);

mod acmd;
mod opff;
mod status;

mod volley;

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

#[skyline::from_offset(0x3ac560)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    let agent = &mut Agent::new("ganon");
    acmd::install(agent);
    opff::install();
    status::install(agent);
    agent.install();

    volley::install();

    unsafe {
        VOLLEY += smashline::clone_weapon("ryu", *WEAPON_KIND_RYU_HADOKEN, "ganon", "volley", false);
    }
}
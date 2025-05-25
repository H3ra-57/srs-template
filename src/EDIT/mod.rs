use std::{hash, task::ready};

use super::*;


pub const CURRENT_FRAME: i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;

pub const MOTION : [u64; 16] = [hash40("attack_11"), hash40("attack_12"), hash40("attack_13"), hash40("attack_dash"), 
                                   hash40("attack_s3_s"), hash40("attack_s3_s2"), hash40("attack_hi3"), hash40("attack_lw3"), 
                                   hash40("attack_s4_s"), hash40("attack_hi4"), hash40("attack_lw4"),
                                   hash40("attack_air_n"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_hi"), hash40("attack_air_lw")];


// Char opff, Global opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    
}

#[skyline::hook(offset = 0x68d7e0)]
unsafe extern "C" fn snake_on_hit(vtable: u64, fighter: &mut Fighter, damage: f32, fighter_kind: i32) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let motion = MotionModule::motion_kind(module_accessor);
    let kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if kind == *FIGHTER_KIND_SNAKE {
        if MOTION.contains(&motion) {
            let heal = damage * -0.5;
            DamageModule::add_damage(module_accessor, heal, 0);
        }
    }
    original!()(vtable, fighter, damage, fighter_kind);
}

pub fn install() {
    /*Agent::new("snake")
        .on_line(Main, fighter_frame) // Char opff
        .install();*/

    skyline::install_hooks!(
        snake_on_hit
    );
}

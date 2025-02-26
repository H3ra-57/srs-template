use std::{hash, task::ready};

use super::*;
use skyline::hooks::{getRegionAddress, Region, InlineCtx};


pub const CURRENT_FRAME: i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;

static mut ARROW_TYPE: [i32; 8] = [0; 8];
static mut TAUNT_EFFECT: [bool; 8] = [false; 8];

// taunts; set arrow type with these
unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if ARROW_TYPE[entry_id] == 1 {
            ARROW_TYPE[entry_id] = 0;
        }
        else {
            ARROW_TYPE[entry_id] = 1;
        }
    }
}

unsafe extern "C" fn effect_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if TAUNT_EFFECT[entry_id] {
            if ARROW_TYPE[entry_id] == 1 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
            }
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if ARROW_TYPE[entry_id] == 2 {
            ARROW_TYPE[entry_id] = 0;
        }
        else {
            ARROW_TYPE[entry_id] = 2;
        }
    }
}

unsafe extern "C" fn effect_appeals(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if TAUNT_EFFECT[entry_id] {
            if ARROW_TYPE[entry_id] == 2 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
            }
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_sword_appeal"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if ARROW_TYPE[entry_id] == 3 {
            ARROW_TYPE[entry_id] = 0;
        }
        else {
            ARROW_TYPE[entry_id] = 3;
        }
    }
}

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if TAUNT_EFFECT[entry_id] {
            if ARROW_TYPE[entry_id] == 3 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
            }
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 12, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

// charge effects
unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_arrow_max"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        if ARROW_TYPE[entry_id] == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        }
        else if ARROW_TYPE[entry_id] == 2 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        }
        else if ARROW_TYPE[entry_id] == 3 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 15, 0, 0, 0, false);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 15, 0, 0, 0, false);
    }
    wait(lua_state, 5.0);
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_arrow_max"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        if ARROW_TYPE[entry_id] == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        }
        else if ARROW_TYPE[entry_id] == 2 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        }
        else if ARROW_TYPE[entry_id] == 3 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("havel"), 0, 0.6, 2.3, 0, 0, 0, 1, true);
        }
    }
    wait(lua_state, 11.0);
}

// arrow hitboxes
unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entry_id = smash::app::lua_bind::WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let arrow_type = ARROW_TYPE[entry_id];

    let element = if arrow_type == 1 { Hash40::new("collision_attr_fire") } else if arrow_type == 2 { Hash40::new("collision_attr_ice") } else if arrow_type == 3 { Hash40::new("collision_attr_paralyze") } else { Hash40::new("collision_attr_sting") };
    let sound = if arrow_type == 1 { *COLLISION_SOUND_ATTR_FIRE } else if arrow_type == 2 { *COLLISION_SOUND_ATTR_FREEZE } else if arrow_type == 3 { *COLLISION_SOUND_ATTR_ELEC } else { *COLLISION_SOUND_ATTR_CUTUP };
    let angle = if arrow_type == 1 { 20 } else { 361 };
    let bkb = if arrow_type == 1 { 30 } else { 0 };
    let kbg = if arrow_type == 1 { 20 } else { 0 };
    let hitlag = if arrow_type == 3 { 0.5 } else { 0.8 };
    if WorkModule::get_int(boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_SHOOT_NUM) <= 0 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 71 + kbg, 0, 10 + bkb, 1.35, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_OBJECT);
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 110 + kbg, 0, 25 + bkb, 1.35, 0.0, 0.0, 0.0, None, None, None, hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, element, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(boma);
            }
        }
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entry_id = smash::app::lua_bind::WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
        if ARROW_TYPE[entry_id] == 1 {
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else if ARROW_TYPE[entry_id] == 2 {
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 1.0);
        }
        else if ARROW_TYPE[entry_id] == 3 {
            LAST_EFFECT_SET_COLOR(agent, 0.2, 1.0, 0.2);
        }
        EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
        if ARROW_TYPE[entry_id] == 1 {
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
            EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 0.5, true);
        }
        else if ARROW_TYPE[entry_id] == 2 {
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 0.35, true);
        }
        else if ARROW_TYPE[entry_id] == 3 {
            LAST_EFFECT_SET_COLOR(agent, 0.2, 1.0, 0.2);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 0.5, true);
        }
    }
}

// opff
unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let lua_state = fighter.lua_state_agent;
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let situation_kind = StatusModule::situation_kind(boma);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
	if [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
		ARROW_TYPE[entry_id] = 0;
        TAUNT_EFFECT[entry_id] = false;
    }
    if [hash40("appeal_hi_r"), hash40("appeal_hi_l"), hash40("appeal_s_r"), hash40("appeal_s_l"), hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
        TAUNT_EFFECT[entry_id] = true;
    }
    else {
        TAUNT_EFFECT[entry_id] = false;
    }
}

#[skyline::hook(offset=0x4E53A0)]
pub unsafe fn get_param_int_hook(x0: u64, x1: u64, x2 :u64) -> i32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = smash::app::utility::get_kind(boma_reference);
    if smash::app::utility::get_category(boma_reference) == *BATTLE_OBJECT_CATEGORY_WEAPON { 
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if fighter_kind == *WEAPON_KIND_LINK_BOWARROW {
            let id = smash::app::lua_bind::WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            if ARROW_TYPE[id] == 1 {
                if x1 == hash40("param_bowarrow") {
                    if x2 == hash40("power_min") {
                        return 6;
                    }
                    else if x2 == hash40("power_max") {
                        return 20;
                    }
                }
            }
        }
    }

    original!()(x0, x1, x2)
}

pub fn install() {
    Agent::new("link")
        .game_acmd("game_appealhir", game_appealhi, Priority::Low)
        .game_acmd("game_appealhil", game_appealhi, Priority::Low)
        .effect_acmd("effect_appealhir", effect_appealhi, Priority::Low)
        .effect_acmd("effect_appealhil", effect_appealhi, Priority::Low)
        .game_acmd("game_appealsr", game_appeals, Priority::Low)
        .game_acmd("game_appealsl", game_appeals, Priority::Low)
        .effect_acmd("effect_appealsr", effect_appeals, Priority::Low)
        .effect_acmd("effect_appealsl", effect_appeals, Priority::Low)
        .game_acmd("game_appeallwr", game_appeallw, Priority::Low)
        .game_acmd("game_appeallwl", game_appeallw, Priority::Low)
        .effect_acmd("effect_appeallwr", effect_appeallw, Priority::Low)
        .effect_acmd("effect_appeallwl", effect_appeallw, Priority::Low)
        .effect_acmd("effect_specialairn", effect_specialairn, Priority::Low)
        .effect_acmd("effect_specialn", effect_specialn, Priority::Low)
        .on_line(Main, link_frame) // Char opff
        .install();
    Agent::new("link_bowarrow")
        .game_acmd("game_fly", game_fly, Priority::Low)
        .effect_acmd("effect_fly", effect_fly, Priority::Low)
        .install();

    skyline::install_hooks!(
        get_param_int_hook,
    );
}

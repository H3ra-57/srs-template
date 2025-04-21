use std::{f32::MIN_10_EXP, hash, task::ready};

use super::*;


pub const CURRENT_FRAME: i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;

static mut WARP_FRAME : [i32; 8] = [0; 8];

// Game acmd script
unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 31.0, 9.0);
    frame(lua_state, 31.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 12.0, 93, 100, 130, 0, 4.8, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 93, 100, 130, 0, 5.8, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE(agent, 0.2);
    if is_excute(agent) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if app::sv_kinetic_energy::get_speed_y(lua_state) < 0.0 {
            KineticModule::mul_speed(boma, &Vector3f{x: 1.0, y: 0.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    FT_MOTION_RATE_RANGE(agent, 1.0, 31.0, 9.0);
    frame(lua_state, 31.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 12.0, 93, 100, 110, 0, 4.8, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 93, 100, 110, 0, 5.8, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE(agent, 0.2);
    if is_excute(agent) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16, -0.5, 0, 20, 90, 1.3, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 6, 15, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..2 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 31.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h01"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

unsafe extern "C" fn game_specialhicatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
}

unsafe extern "C" fn game_specialhithrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 10.0, 270, 78, 0, 30, 6.0, 0.0, -2.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 8.0, 270, 78, 0, 30, 6.0, 0.0, 2.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhithrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 6, 15, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_raijin_bomb"), Hash40::new("top"), 0, 12, -4, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 30, 2, 90, 180, 0, 1.5, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.8, 0.6, 3);
    }
}


// Status script
unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_AIR as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 
        *FIGHTER_STATUS_ATTR_START_TURN as u32, 
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 
        0
    );
    0.into()
}

unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.1);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.1);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.05);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    0.into()
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0,1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0,1.0, false, 0.0, false, false);
    }
    WorkModule::set_float(fighter.module_accessor, 11.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() 
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING.into(),false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_warp_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WARP_FRAME[entry_id] = 0;
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_FREE, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        false, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );
    
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32, 
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 
        0);
        
    0.into()
}

unsafe extern "C" fn special_hi_warp_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    JostleModule::set_status(fighter.module_accessor, false);
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut stick_x =  ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let mut stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
    let warp_stick = 0.3;
    if (stick_x.abs() < warp_stick && stick_y.abs() < warp_stick) {
        stick_x = 0.0;
        stick_y = 1.0;
    }
    else if (stick_y.abs() < warp_stick) {
        stick_y = 0.0;
    }
    else if (stick_x.abs() < warp_stick) {
        stick_x = 0.0;
    }
    let normalized = sv_math::vec2_normalize(stick_x, stick_y);
    let speed_x= normalized.x * 2.7 * lr;
    let mut speed_y= normalized.y * 2.7;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        stick_y = stick_y.max(0.0);
        if stick_y > warp_stick {
            fighter.set_situation(SITUATION_KIND_AIR.into());
        }
        if stick_y < warp_stick {
            speed_y = 0.0;
        }
    }
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    0.into()
}

unsafe extern "C" fn special_hi_warp_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::set_passable_check(fighter.module_accessor, true);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    WorkModule::set_flag(fighter.module_accessor, false,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_catch"), 0.0,1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_warp_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_warp_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = WARP_FRAME[entry_id] as f32;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let max_frame_int = frame > 20 as f32;
    let min_frame_int = frame > 12 as f32;
    let stop_ground = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND &&
    fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && sum_speed_y < 0.0;
    let stop_air = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && sum_speed_y < 0.0 && (
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) ||
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) ||
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) ||
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    );
    if max_frame_int || stop_ground || stop_air {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW.into(),false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn special_hi_warp_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WARP_FRAME[entry_id] += 1;
    0.into() 
}

unsafe extern "C" fn special_hi_warp_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    0.into()
}

unsafe extern "C" fn special_hi_warp_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = smash::app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WARP_FRAME[entry_id] = 0;
    0.into()
}

unsafe extern "C" fn special_hi_slam_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 
        0
    );
    
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 
        0, 
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 
        0
    );
    
    0.into()
}

unsafe extern "C" fn special_hi_slam_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0,1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: 0.0, y: -4.0, z: 0.0 });
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_throw"), 0.0,1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_slam_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_slam_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
        && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi_slam_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialhi", game_specialhi, Priority::Low)
        .game_acmd("game_specialairhi", game_specialairhi, Priority::Low)
        .effect_acmd("effect_specialhi", effect_specialhi, Priority::Low)
        .effect_acmd("effect_specialairhi", effect_specialhi, Priority::Low)
        .sound_acmd("sound_specialhi", sound_specialhi, Priority::Low)
        .sound_acmd("sound_specialairhi", sound_specialhi, Priority::Low)

        .game_acmd("game_specialhicatch", game_specialhicatch, Priority::Low)

        .game_acmd("game_specialhithrow", game_specialhithrow, Priority::Low)
        .effect_acmd("effect_specialhithrow", effect_specialhithrow, Priority::Low)
        
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre) 
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end)

        .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, special_hi_warp_pre)
        .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, special_hi_warp_init)
        .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, special_hi_warp_main)
        .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, special_hi_warp_exec)
        .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, special_hi_warp_exit)
        .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, special_hi_warp_end)

        .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, special_hi_slam_pre)
        .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, special_hi_slam_main)
        .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, special_hi_slam_end)
        .install();
}

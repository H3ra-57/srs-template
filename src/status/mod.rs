use std::{hash, task::ready};

use super::*;


pub const STATUS_KIND: i32 = 0xB;
pub const CURRENT_FRAME: i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;

pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT: i32 = 0x1f0;

// Status script
unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    //println!("situation: {}", fighter.global_table[SITUATION_KIND].get_i32());
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.5);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    return 0.into();
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0,1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0,1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if frame >= 3.0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.06);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        //println!("the floaterr");
    }


    if !StatusModule::is_changing(fighter.module_accessor) 
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == (*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.06);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
        }
        else {    
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.5);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.06);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
        }

    }

    if MotionModule::is_end(fighter.module_accessor) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 1.into();
    }

    if frame >= 30.0 && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let effect = WorkModule::get_int(fighter.module_accessor, *VOLLEY_EFFECT);
    if frame >= 30.0 {
        WorkModule::add_float(fighter.module_accessor, 0.05, *VOLLEY_DAMAGE);
        WorkModule::add_float(fighter.module_accessor, 0.01667, *VOLLEY_SCALE);
        let scale = WorkModule::get_float(fighter.module_accessor, *VOLLEY_SCALE);
        EffectModule::set_scale(fighter.module_accessor, effect as u32, &Vector3f{x: 1.0+scale, y: 1.0+scale, z: 1.0+scale});
    }
    0.into()
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    STOP_SE(fighter, Hash40::new("se_item_superscope_charge"));
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_sscope_bullet_max"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_shield_smoke"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *VOLLEY_EFFECT);
    if fighter.global_table[STATUS_KIND].get_i32() != FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT {
        WorkModule::set_float(fighter.module_accessor, 0.0, *VOLLEY_DAMAGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, *VOLLEY_SCALE);
    }
    0.into()
}

unsafe extern "C" fn special_n_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_shoot"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_shoot"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    fighter.sub_air_check_dive();

    if !StatusModule::is_changing(fighter.module_accessor) 
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_shoot"), -1.0, 1.0, 0.0, false, false);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_n_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *VOLLEY_DAMAGE);
    WorkModule::set_float(fighter.module_accessor, 0.0, *VOLLEY_SCALE);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);

    agent.status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_pre);
    agent.status(Init, FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_init);
    agent.status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main);
    agent.status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_end);
}

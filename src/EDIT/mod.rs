use std::{hash, task::ready};

use super::*;


pub const CURRENT_FRAME:                    i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;


// SPECIAL_S
unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
    return 0.into();
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// SPECIAL_S_BLOW
pub unsafe extern "C" fn special_s_blow_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_s_blow_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_blow"), 0.0, 1.0, false, 0.0, false, false);
    /*if StopModule::is_stop(fighter.module_accessor) {
    }*/
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_blow_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_blow_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        // println!("AIR_CLIFF main_loop");
        return 1.into()
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        // println!("CANCEL main_loop");
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            // println!("WAIT_GROUND main_loop");
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            if WorkModule::get_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME) >= 0 {
                fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END.into(), false.into());
            }
        }
    }
    else {
        special_s_ray_check_helper(fighter);
    }
    return 0.into();
}

    // if not motion end
        // special_s_ray_check_helper
    // else if air, fall, else if ground, special_s_blow_end
        
    /*if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            // LAB_71000199bc:
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                // LAB_7100019a94:
                if !MotionModule::is_end(fighter.module_accessor) {
                    special_s_ray_check_helper(fighter);
                    return 0.into(); // goto LAB_7100019b18;
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                if WorkModule::get_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME) >= 0 {
                    // goto LAB_7100019a94;
                    if !MotionModule::is_end(fighter.module_accessor) {
                        special_s_ray_check_helper(fighter);
                        return 0.into(); // goto LAB_7100019b18;
                    }
                } 
                fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END.into(), false.into());
            }
        }
        else {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if !fighter.sub_air_check_fall_common().get_bool() { 
                    return 1.into(); // goto LAB_7100019b10;
                }
                // goto LAB_71000199bc;
                if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                    // LAB_7100019a94:
                    if !MotionModule::is_end(fighter.module_accessor) {
                        special_s_ray_check_helper(fighter);
                        return 0.into(); // goto LAB_7100019b18;
                    }
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                else {
                    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME) >= 0 {
                        // goto LAB_7100019a94;
                    } 
                    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END.into(), false.into());
                }
            }
        }
    }
    return 1.into();
}*/
unsafe extern "C" fn special_s_ray_check_helper(fighter: &mut L2CFighterCommon) {  
    let ray_check_start_frame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_ray_check_start_frame_"));
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT) > ray_check_start_frame - 1 { // goto LAB_710001a1d8;
        return;
    } 
    let is_ray_check_result = WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
    let zero_vec = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let mut pLVar7 = zero_vec.x;
    let mut this = zero_vec.y;
    let mut this_00 = zero_vec.z;
    let pos = PostureModule::pos(fighter.module_accessor);
    pLVar7 = (*pos).x;
    this = (*pos).y;
    this_00 = (*pos).z;
    let mut LStack_110 = 0.0;
    let pLVar9 = zero_vec.y;
    if WorkModule::get_float(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y) < pLVar9 {
        pLVar7 = zero_vec.y;
        LStack_110 = pLVar7 - WorkModule::get_float(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    }
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
    let local_100 = Vector2f{ x: sum_speed_x, y: sum_speed_y };
    let local_70 = Vector2f{ x: 0.0, y: -1.0-LStack_110 };
    if GroundModule::ray_check(fighter.module_accessor,&local_100,&local_70,true) == 1 {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
    // LAB_710001a1a8:↑↑
    }
    else {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
        if is_ray_check_result {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP,fighter.module_accessor);
            // goto LAB_710001a1a8;↑↑
        }
    }
    // LAB_710001a1d8:↑
    return;
}

unsafe extern "C" fn special_s_blow_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// SPECIAL_S_BLOW_END
unsafe extern "C" fn special_s_blow_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_s_blow_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);

    let prev_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, prev_speed_x, 0.0);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let mut prev_motion_frame = MotionModule::frame(fighter.module_accessor);
    if prev_motion_frame - 10.0 < 0.0 {
        prev_motion_frame = 0.0;
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_blow_end"), prev_motion_frame, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_blow_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_blow_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_blow_landing_frame_"));
    if MotionModule::is_end(fighter.module_accessor)
    && fighter.global_table[CURRENT_FRAME].get_i32() >= landing_frame {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn special_s_blow_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// SPECIAL_S_JUMP
unsafe extern "C" fn special_s_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_STATUS_SPECIAL_S_FLAG_IS_BLOW_SHIFT);

    let mut landing_frame = 0;
    let mut landing_disable_frame = 0;
    let mut jump_brake_x = 0.0;
    let mut jump_x_speed = 0.0;
    let mut jump_y_speed = 0.0;
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_jump"), 0.0, 1.0, false, 0.0, false, false);
        
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        jump_x_speed = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_x_speed_air_")) * PostureModule::lr(fighter.module_accessor);
        jump_y_speed = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_y_speed_air_"));
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        jump_x_speed = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_x_speed_")) * PostureModule::lr(fighter.module_accessor);
        jump_y_speed = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_y_speed_"));
    }
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, jump_x_speed, 0.0);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    jump_brake_x = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_brake_x_"));
    
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, jump_brake_x, 0.0);
    app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    
    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    let fVar10 = -WorkModule::get_float(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_AIR_ACCEL_Y);
    
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fVar10, 0.0);
    app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    landing_frame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_end_landing_frame_"));
    WorkModule::set_float(fighter.module_accessor,landing_frame as f32,*FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    landing_disable_frame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_landing_disable_frame_"));
    WorkModule::set_int(fighter.module_accessor,landing_disable_frame,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME);
        WorkModule::inc_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    }
    else {
        WorkModule::dec_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME);
        WorkModule::inc_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    }
    
    WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    
    
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    let pos = PostureModule::pos(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, (*pos).y, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_jump_main_loop as *const () as _))

}

/*lib::L2CValue::L2CValue(&LStack_168,0.0);
    lib::L2CValue::L2CValue(&LStack_178,0.0);
    lib::L2CValue::L2CValue(&LStack_188,0.0);
    lua2cpp::L2CFighterBase::Vector3::create(this,SUB81(&LStack_168,0),SUB81(&LStack_178,0),SUB81(&LStack_188,0));
    pLVar4 = (L2CValue *)lib::L2CValue::operator[](&LStack_d8,x);
    this_00 = (L2CValue *)lib::L2CValue::operator[](&LStack_d8,y);
    this_01 = (L2CValue *)lib::L2CValue::operator[](&LStack_d8,z);
    pfVar8 = PostureModule::pos(fighter.module_accessor);
    lib::L2CValue::L2CValue(&LStack_1b8,*pfVar8);
    lib::L2CValue::L2CValue(&LStack_1a8,pfVar8[1]);
    lib::L2CValue::L2CValue(&LStack_198,pfVar8[2]);
    lib::L2CValue::operator=(pLVar4,(L2CValue *)&LStack_1b8);
    lib::L2CValue::operator=(this_00,(L2CValue *)&LStack_1a8);
    lib::L2CValue::operator=(this_01,(L2CValue *)&LStack_198);
    pLVar4 = (L2CValue *)lib::L2CValue::operator[](&LStack_d8,y);
    let zero_vector = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let pLVar4 = zero_vector.x;
    let this_00 = zero_vector.y;
    let this_01 = zero_vector.z;
    let pos = PostureModule::pos(fighter.module_accessor);
    let LStack_1b8 = pos.x; 
    let LStack_1a8 = pos.y;
    let LStack_198 = pos.z;
    pLVar4 = LStack_1b8;
    this_00 = LStack_1a8;
    this_01 = LStack_198;
    pLVar4 = LStack_1a8;
    lib::L2CValue::L2CValue(&LStack_1b8,0.0);
    lib::L2CValue::operator+(pLVar4,(L2CValue *)&LStack_1b8,(L2CValue *)&LStack_e8);
    lib::L2CValue::L2CValue(&LStack_1b8,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    fVar10 = (float)lib::L2CValue::as_number(&LStack_e8);
    iVar2 = lib::L2CValue::as_integer(&LStack_1b8);
    WorkModule::set_float(fighter.module_accessor,fVar10,iVar2);
    lib::L2CValue::L2CValue(&LStack_1b8,0);
    lib::L2CValue::L2CValue(&LStack_e8,_FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    iVar2 = lib::L2CValue::as_integer(&LStack_1b8);
    iVar3 = lib::L2CValue::as_integer(&LStack_e8);
    WorkModule::set_int(fighter.module_accessor,iVar2,iVar3);
    lib::L2CValue::L2CValue(&LStack_1b8,_FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
    iVar2 = lib::L2CValue::as_integer(&LStack_1b8);
    WorkModule::off_flag(fighter.module_accessor,iVar2);
    lib::L2CValue::L2CValue(&LStack_1c8,&LAB_710001bc10);
    lua2cpp::L2CFighterCommon::sub_shift_status_main(this,SUB81(&LStack_1c8,0));*/

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        if WorkModule::get_int(fighter.module_accessor,*FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME) < 0 {
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x, 0.0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            
            KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW.into(), false.into());
        return 1.into();
    }
    let mut frame = false;
    if MotionModule::frame(fighter.module_accessor) > WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_s"),hash40("special_s_jump_attack_shift_frame_min_")) as f32 {
        if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
            frame = fighter.global_table[CURRENT_FRAME].get_i32() >= 0;
        }
        else {
            if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
                frame = fighter.global_table[CURRENT_FRAME].get_i32() >= 0;
                return 0.into();
            }
        }
    }
    
    if frame {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_STATUS_SPECIAL_S_FLAG_IS_BLOW_SHIFT);
    }
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LITTLEMAC_STATUS_SPECIAL_S_FLAG_IS_BLOW_SHIFT) {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW.into(), false.into());
        return 1.into();
    }
    special_s_ray_check_helper(fighter);
    return 0.into();
}

unsafe extern "C" fn special_s_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into()
}

// SPECIAL_S_JUMP_END
unsafe extern "C" fn special_s_jump_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_s_jump_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_jump_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_jump_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn special_s_jump_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into()
}

pub fn install() {
    Agent::new("littlemac")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end)

        .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_pre)
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main)
        .status(End, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_end)

        .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END, special_s_jump_end_pre)
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END, special_s_jump_end_main)
        .status(End, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END, special_s_jump_end_end)

        .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW, special_s_blow_pre)
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW, special_s_blow_main)
        .status(End, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW, special_s_blow_end)

        .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END, special_s_blow_end_pre)
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END, special_s_blow_end_main)
        .status(End, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END, special_s_blow_end_end)
        .install();
}

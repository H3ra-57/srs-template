use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

pub const MODULE_ACCESSOR:                 i32 = 0x5;
pub const SUB_STATUS:                      i32 = 0x15;
pub const SITUATION_KIND:                  i32 = 0x16;

// Status script
unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let explosion_speed_coef = WorkModule::get_param_float(fighter.module_accessor, hash40::new("param_special_s"), hash40::new("explosion_speed_coef"));
    WorkModule::set_float(fighter.module_accessor, explosion_speed_coef, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        if !StopModule::is_stop(fighter.module_accessor) {
            lib::L2CValue::L2CValue(&LStack_80,false);
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF) {
                KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_substatus as *const () as _));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor,*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
        if GrabModule::is_grab(fighter.module_accessor, 0) {
            MotionModule::set_frame(fighter.module_accessor,30.0 ,true);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        }
        GrabModule::clear_all(fighter.module_accessor);
        AttackModule::clear_all(fighter.module_accessor);
    }

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION) != *SITUATION_KIND_GROUND {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF) {
                WorkModule::set_float(fighter.module_accessor,0.0,*FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    // LAB_710001651c:
    // pLVar6 = &LStack_50;
        
}
    // LAB_710001652c:
    // iVar3 = 1;
    // LAB_7100016534:
    // lib::L2CValue::L2CValue(return_value,iVar3);
    // return;

    
    /*if WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION) != *SITUATION_KIND_GROUND {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF) {
                    WorkModule::set_float(fighter.module_accessor,0.0,*FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    // goto LAB_710001651c;
                    // pLVar6 = &LStack_50;
                }
            }
            // LAB_7100016650:
            // iVar3 = 0;
            // goto LAB_7100016534;
            // lib::L2CValue::L2CValue(return_value,iVar3);
            return 0.into();
        }
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            if !MotionModule::is_end(fighter.module_accessor) {
                // goto LAB_7100016650;
                // iVar3 = 0;
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }*/

pub fn install() {
    Agent::new("ganon")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main) // Status script
        .install();
}

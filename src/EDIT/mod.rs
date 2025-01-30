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

pub const SUB_STATUS:                      i32 = 0x15;
pub const SITUATION_KIND:                  i32 = 0x16;

// Status script
unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let explosion_speed_coef = WorkModule::get_param_float(fighter.module_accessor, hash40::new("param_special_s"), hash40::new("explosion_speed_coef"), uVar4, uVar5);
    WorkModule::set_float(fighter.module_accessor, explosion_speed_coef, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    pLVar6 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
    lib::L2CValue::L2CValue(&LStack_60,_SITUATION_KIND_GROUND);
    uVar4 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack_60);
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        FUN_7100010bc0(this);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        bVar1 = ;
        lib::L2CValue::L2CValue(&LStack_70,(bool)(bVar1 & 1));
        lib::L2CValue::L2CValue(&LStack_60,false);
        uVar4 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
        if !StopModule::is_stop(fighter.module_accessor) {
            lib::L2CValue::L2CValue(&LStack_80,false);
            FUN_7100015fe0(false,this,false);
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_substatus as *const () as _));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        FUN_7100010b20(this);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            // LAB_7100016280:
            if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
                if GrabModule::is_grab(fighter.module_accessor, 0) {
                    lib::L2CValue::L2CValue(&LStack_50,30.0);
                    fVar8 = (float)lib::L2CValue::as_number(&LStack_50);
                    MotionModule::set_frame(fighter.module_accessor,fVar8,true);
                    lib::L2CValue::L2CValue(&LStack_50,FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    pLVar6 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,5);
                    iVar3 = lib::L2CValue::as_integer(&LStack_50);
                    pBVar7 = (BattleObjectModuleAccessor *)lib::L2CValue::as_pointer(pLVar6);
                    app::KineticUtility::clear_unable_energy(iVar3,pBVar7);
                }
                GrabModule::clear_all(fighter.module_accessor);
                AttackModule::clear_all(fighter.module_accessor);
            }
            iVar3 = _SITUATION_KIND_GROUND;
            lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
            iVar4 = lib::L2CValue::as_integer(&LStack_70);
            iVar4 = WorkModule::get_int(fighter.module_accessor,iVar4);
            lib::L2CValue::L2CValue(&LStack_60,iVar4);
            lib::L2CValue::L2CValue(&LStack_50,iVar3);
            uVar5 = lib::L2CValue::operator==(&LStack_50,(L2CValue *)&LStack_60);
            if ((uVar5 & 1) == 0) {
                bVar2 = MotionModule::is_end(fighter.module_accessor);
                lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar2 & 1));
                bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
                if (!bVar1) {
                    pLVar6 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
                    lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
                    uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack_50);
                    if ((uVar5 & 1) != 0) {
                        lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
                        iVar3 = lib::L2CValue::as_integer(&LStack_60);
                        bVar2 = WorkModule::is_flag(fighter.module_accessor,iVar3);
                        lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar2 & 1));
                        bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
                        if (bVar1) {
                            lib::L2CValue::L2CValue(&LStack_50,0.0);
                            lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                            fVar8 = (float)lib::L2CValue::as_number(&LStack_50);
                            iVar3 = lib::L2CValue::as_integer(&LStack_60);
                            WorkModule::set_float(fighter.module_accessor,fVar8,iVar3);
                            lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
                            lib::L2CValue::L2CValue(&LStack_60,false);
                            lua2cpp::L2CFighterBase::change_status(this,SUB81(&LStack_50,0),SUB81(&LStack_60,0));
                            // goto LAB_710001651c;
                        }
                    }
                    // LAB_7100016650:
                    iVar3 = 0;
                    // goto LAB_7100016534;
                }
                lib::L2CValue::L2CValue(&LStack_50,FIGHTER_STATUS_KIND_FALL_SPECIAL);
                lib::L2CValue::L2CValue(&LStack_60,false);
                lua2cpp::L2CFighterBase::change_status(this,SUB81(&LStack_50,0),SUB81(&LStack_60,0));
            }
            else {
                pLVar6 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
                lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
                uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack_50);
                if ((uVar5 & 1) == 0) {
                    lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_FALL);
                    lib::L2CValue::L2CValue(&LStack_60,false);
                    lua2cpp::L2CFighterBase::change_status(this,SUB81(&LStack_50,0),SUB81(&LStack_60,0));
                }
                else {
                    bVar2 = MotionModule::is_end(fighter.module_accessor);
                    lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar2 & 1));
                    bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
                    if (!bVar1) // goto LAB_7100016650;
                    lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_WAIT);
                    lib::L2CValue::L2CValue(&LStack_60,false);
                    lua2cpp::L2CFighterBase::change_status(this,SUB81(&LStack_50,0),SUB81(&LStack_60,0));
                }
            }
            // LAB_710001651c:
            pLVar6 = &LStack_50;
        }
        else {
            lib::L2CValue::L2CValue(&LStack_80,false);
            lua2cpp::L2CFighterCommon::sub_wait_ground_check_common(this,SUB81(&LStack_80,0));
            lib::L2CValue::L2CValue(&LStack_50,false);
            uVar5 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_50);
            if ((uVar5 & 1) != 0) {
                lua2cpp::L2CFighterCommon::sub_air_check_fall_common(this);
                lib::L2CValue::L2CValue(&LStack_50,false);
                uVar5 = lib::L2CValue::operator==(&LStack_90,(L2CValue *)&LStack_50);
                if ((uVar5 & 1) == 0) // goto LAB_710001652c;
                // goto LAB_7100016280;
            }
            pLVar6 = &LStack_60;
        }
    }
    // LAB_710001652c:
    iVar3 = 1;
    // LAB_7100016534:
    lib::L2CValue::L2CValue(return_value,iVar3);
    return;
}

pub fn install() {
    Agent::new("ganon")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, example_status_script) // Status script
        .install();
}

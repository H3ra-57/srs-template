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

pub const SUB_STATUS = 0x15;
pub const SITUATION_KIND = 0x16;


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
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_BOTH_SIDES),
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

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR)
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND)
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END)
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1)
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_blow"), 0.0, 1.0, false, 0.0, false, false);
    /*if StopModule::is_stop(fighter.module_accessor) {
    }*/
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_blow_sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_blow_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_blow_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) { 
            // LAB_71000199bc:↑↑
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                // LAB_7100019a94:↑↑↑
                if !MotionModule::is_end(fighter.module_accessor) {
                    // FUN_7100019c20(this);
                    return 0.into();
                } // end LAB_7100019a94:↑↑↑
                fighter.change_status(*FIGHTER_STATUS_KIND_FALL.into(), false.into());
            } // end LAB_71000199bc
            else {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME) >= 0 {
                    if !MotionModule::is_end(fighter.module_accessor) {
                        // FUN_7100019c20(this);
                        return 0.into();
                    }
                }
                fighter.change_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END.into(), false.into());
            }
            pLVar5 = &LStack_40;
        }
        else { // if enable cancel
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() { // if sub wait ground check common
                if !fighter.sub_air_check_fall_common(false.into()).get_bool() {
                    iVar3 = 1;
                } 
                if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
                    // LAB_7100019a94:↑↑↑
                    if !MotionModule::is_end(fighter.module_accessor) {
                        // FUN_7100019c20(this);
                        return 0.into();
                    } // end LAB_7100019a94:↑↑↑
                    fighter.change_status(*FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
            }
            pLVar5 = &LStack_50;
        }
    }
    // LAB_7100019b10:↑↑↑↑
    iVar3 = 1;
    // LAB_7100019b18:↑
    lib::L2CValue::L2CValue(return_value,iVar3);
    return;
}
unsafe extern "C" fn FUN_7100019c20(fighter: &mut L2CFighterCommon)

{  
    let ray_check_start_frame = WorkModule::get_param_int(fighter.module_accessor, hash40::new("param_special_s"), hash40::new("special_s_ray_check_start_frame_"));
    lib::L2CValue::L2CValue(&LStack_80,iVar4);
    lib::L2CValue::L2CValue(&LStack_90,_FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    iVar4 = lib::L2CValue::as_integer(&LStack_90);
    iVar4 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    lib::L2CValue::L2CValue((L2CValue *)&local_70,iVar4);
    lib::L2CValue::L2CValue((L2CValue *)&local_100,1);
    lib::L2CValue::operator-(&LStack_80,(L2CValue *)&local_100);
    uVar5 = lib::L2CValue::operator<=(&LStack_a0,(L2CValue *)&local_70);
    if ((uVar5 & 1) == 0) // goto LAB_710001a1d8;↑
    lib::L2CValue::L2CValue
            ((L2CValue *)&local_100,
             _FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
    iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_100);
    bVar2 = WorkModule::is_flag
                    (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
    lib::L2CValue::L2CValue(&LStack_90,(bool)(bVar2 & 1));
    lib::L2CValue::L2CValue(&LStack_b0,0.0);
    lib::L2CValue::L2CValue(&LStack_c0,0.0);
    lib::L2CValue::L2CValue(&LStack_d0,0.0);
    cVar1 = (char)&stack0xfffffffffffffff0;
    lua2cpp::L2CFighterBase::Vector3::create
            (param_1,(L2CValue)(cVar1 + '`'),(L2CValue)(cVar1 + 'P'),(L2CValue)(cVar1 + '@'));
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    this = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
    this_00 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x162d277af);
    pfVar8 = (float *)PostureModule::pos
                              (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40));
    lib::L2CValue::L2CValue((L2CValue *)&local_100,*pfVar8);
    lib::L2CValue::L2CValue(&LStack_f0,pfVar8[1]);
    lib::L2CValue::L2CValue(&LStack_e0,pfVar8[2]);
    lib::L2CValue::operator=(pLVar7,(L2CValue *)&local_100);
    lib::L2CValue::operator=(this,(L2CValue *)&LStack_f0);
    lib::L2CValue::operator=(this_00,(L2CValue *)&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_110,0.0);
    pLVar9 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
    lib::L2CValue::L2CValue
            ((L2CValue *)&local_70,_FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_70);
    fVar11 = (float)WorkModule::get_float
                            (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
    lib::L2CValue::L2CValue((L2CValue *)&local_100,fVar11);
    uVar5 = lib::L2CValue::operator<((L2CValue *)&local_100,pLVar9);
    if ((uVar5 & 1) != 0) {
        pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
        lib::L2CValue::L2CValue(&LStack_120,_FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y)
        ;
        iVar4 = lib::L2CValue::as_integer(&LStack_120);
        fVar11 = (float)WorkModule::get_float
                              (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
        lib::L2CValue::L2CValue((L2CValue *)&local_70,fVar11);
        lib::L2CValue::operator-(pLVar7,(L2CValue *)&local_70);
        lib::L2CValue::operator=(&LStack_110,(L2CValue *)&local_100);
    }
    lib::L2CValue::L2CValue((L2CValue *)&local_100,_KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_100);
    fVar11 = (float)KineticModule::get_sum_speed_x
                            (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
    lib::L2CValue::L2CValue(&LStack_120,fVar11);
    lib::L2CValue::L2CValue((L2CValue *)&local_100,_KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_100);
    fVar11 = (float)KineticModule::get_sum_speed_y
                            (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
    lib::L2CValue::L2CValue(&LStack_130,fVar11);
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    lib::L2CValue::operator+(pLVar7,(L2CValue *)&LStack_120);
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
    lib::L2CValue::operator+(pLVar7,(L2CValue *)&LStack_130);
    lib::L2CValue::L2CValue(&LStack_170,0.0);
    lib::L2CValue::L2CValue((L2CValue *)&local_100,-1.0);
    lib::L2CValue::operator-((L2CValue *)&local_100,(L2CValue *)&LStack_110);
    lib::L2CValue::L2CValue(&LStack_190,true);
    uVar12 = lib::L2CValue::as_number(&LStack_150);
    uVar13 = lib::L2CValue::as_number(&LStack_160);
    local_100._4_4_ = uVar13;
    local_100._0_4_ = uVar12;
    uStack_f8 = 0;
    uVar12 = lib::L2CValue::as_number(&LStack_170);
    uVar13 = lib::L2CValue::as_number(&LStack_180);
    local_70._4_4_ = uVar13;
    local_70._0_4_ = uVar12;
    uStack_68 = 0;
    bVar2 = lib::L2CValue::as_bool(&LStack_190);
    bVar2 = GroundModule::ray_check
                    (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),(Vector2f *)&local_100,
                     (Vector2f *)&local_70,(bool)(bVar2 & 1));
    lib::L2CValue::L2CValue(&LStack_140,(bool)(bVar2 & 1));
    bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack_140);
    if (bVar3) {
        lib::L2CValue::L2CValue
              ((L2CValue *)&local_100,
               _FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
        iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_100);
        WorkModule::on_flag
              (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
    // LAB_710001a1a8:↑↑
    }
    else {
        lib::L2CValue::L2CValue
              ((L2CValue *)&local_100,
               _FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
        iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_100);
        WorkModule::off_flag
                (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),iVar4);
        bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack_90);
        if (bVar3) {
            lib::L2CValue::L2CValue((L2CValue *)&local_100,_FIGHTER_KINETIC_ENERGY_ID_STOP);
            pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)((long)param_1 + 200),5);
            iVar4 = lib::L2CValue::as_integer((L2CValue *)&local_100);
            pBVar10 = (BattleObjectModuleAccessor *)lib::L2CValue::as_pointer(pLVar7);
            app::KineticUtility::clear_unable_energy(iVar4,pBVar10);
            // goto LAB_710001a1a8;↑↑
        }
    }
    // LAB_710001a1d8:↑
    return;
}*/
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
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, prev_speed_x, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let mut prev_motion_frame = MotionModule::frame(fighter.module_accessor);
    if prev_motion_frame - 10.0 < 0.0 {
        prev_motion_frame = 0.0;
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_blow_end"), prev_motion_frame, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_blow_end_main_loop)
}

unsafe extern "C" fn special_s_blow_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_blow_landing_frame_"));
    if MotionModule::is_end(fighter.module_accessor)
    && fighter.global_table[CURRENT_FRAME].get_i32() >= landing_frame {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    // </HDR>
    0.into()
    /*void ::thiscall
L2CFighterLittlemac::status::SpecialSBlowEnd_main_loop
          (L2CFighterLittlemac *this,L2CValue *return_value)

{
  byte bVar1;
  bool bVar2;
  int iVar3;
  ulong uVar4;
  L2CValue *pLVar5;
  ulong uVar6;
  L2CValue *pLVar7;
  L2CValue LStack_90;
  L2CValue LStack_80;
  L2CValue LStack_70;
  L2CValue LStack_60;
  L2CValue LStack_50;
  
  bVar1 = CancelModule::is_enable_cancel(fighter.module_accessor);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  lib::L2CValue::L2CValue(&LStack_50,true);
  uVar4 = lib::L2CValue::operator==(&LStack_60,(L2CValue *)&LStack_50);
  if ((uVar4 & 1) == 0) { if not enable cancel
// LAB_71000194f0:
    bVar1 = MotionModule::is_end(fighter.module_accessor);
    lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar1 & 1));
    bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
    if (bVar2) { if is end
      pLVar5 = (L2CValue *)lib::L2CValue::operator[](fighter.global_table,0xe);
      lib::L2CValue::L2CValue(&LStack_70,0xfea97fe73);
      lib::L2CValue::L2CValue(&LStack_90,0x1dcfe6525f);
      uVar4 = lib::L2CValue::as_integer(&LStack_70);
      uVar6 = lib::L2CValue::as_integer(&LStack_90);
      iVar3 = WorkModule::get_param_int(fighter.module_accessor,uVar4,uVar6);
      lib::L2CValue::L2CValue(&LStack_60,iVar3);
      uVar4 = lib::L2CValue::operator<=(&LStack_60,pLVar5);
      if ((uVar4 & 1) != 0) {
        pLVar7 = (L2CValue *)lib::L2CValue::operator[](fighter.global_table,0x16);
        lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
        uVar4 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack_50);
        if ((uVar4 & 1) == 0) {
          lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_FALL);
          lib::L2CValue::L2CValue(&LStack_60,false);
          lua2cpp::L2CFighterBase::change_status(this,SUB81(&LStack_50,0),SUB81(&LStack_60,0));
        }
        else {
          lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_WAIT);
          lib::L2CValue::L2CValue(&LStack_60,false);
          lua2cpp::L2CFighterBase::change_status(this,SUB81(&LStack_50,0),SUB81(&LStack_60,0));
        }
        pLVar7 = &LStack_50;
        // goto LAB_710001967c;
      }
    }
    else {
    }
    iVar3 = 0;
  }
  else { if enable cancel
    lib::L2CValue::L2CValue(&LStack_80,false);
    lua2cpp::L2CFighterCommon::sub_wait_ground_check_common(this,SUB81(&LStack_80,0));
    lib::L2CValue::L2CValue(&LStack_50,false);
    uVar4 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_50);
    if ((uVar4 & 1) == 0) { if not sub wait
      pLVar7 = &LStack_60;
// LAB_710001967c:
    }
    else { if sub wait
      lua2cpp::L2CFighterCommon::sub_air_check_fall_common(this);
      lib::L2CValue::L2CValue(&LStack_50,false);
      uVar4 = lib::L2CValue::operator==(&LStack_90,(L2CValue *)&LStack_50);
      if ((uVar4 & 1) != 0) // goto LAB_71000194f0; if sub air check fall
    }
    iVar3 = 1;
  }
  lib::L2CValue::L2CValue(return_value,iVar3);
  return;
}*/
}

pub fn install() {
    Agent::new("littlemac")
        .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S, special_s_pre)
        .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S, special_s_main)
        .status(End, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S, special_s_end)
        .install();
}

use std::{hash, task::ready};

use super::*;


pub const FIGHTER_KIND: i32 = 0x2;
pub const OBJECT_ID: i32 = 0x3;
pub const CURRENT_FRAME: i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;

// Status script
unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*pLVar13 = (L2CValue *)lib::L2CValue::operator[](fighter.global_table,2);
    lib::L2CValue::L2CValue(&LStack_78,_FIGHTER_KIND_LINK);
    uVar14 = lib::L2CValue::operator==(pLVar13,(L2CValue *)&LStack_78);
    if fighter.global_table[FIGHTER_KIND] != FIGHTER_KIND_LINK { goto LAB_710001ceec;}
    lib::L2CValue::L2CValue(&LStack_78,_FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
    iVar5 = lib::L2CValue::as_integer(&LStack_78);
    iVar5 = WorkModule::get_int(fighter.module_accessor,iVar5);
    lib::L2CValue::L2CValue(&LStack_88,iVar5);
    uVar6 = lib::L2CValue::as_integer(&LStack_88);
    pvVar15 = (void *)ItemManager::find_active_item_from_id(FIGHTER_STATUS_AIR_LASSO_REACH_WORK_FLOAT_CLIFF_POS_Y,uVar6);
    if (pvVar15 == (void *)0x0) {
      lib::L2CValue::L2CValue(&LStack_98,(L2CValue *)&LUA_SCRIPT_LINE_SYSTEM_POST);
    }
    else {
      lib::L2CValue::L2CValue(&LStack_98,pvVar15);
    }
    uVar14 = lib::L2CValue::operator==(&LStack_98,(L2CValue *)&LUA_SCRIPT_LINE_SYSTEM_POST);
    if ((uVar14 & 1) == 0) {
      pLVar13 = (L2CValue *)lib::L2CValue::operator[](&fighter.global_table,3);
      pIVar16 = (Item *)lib::L2CValue::as_pointer(&LStack_98);
      uVar6 = Item::owner_id(pIVar16);
      lib::L2CValue::L2CValue(&LStack_a8,uVar6);
      uVar14 = lib::L2CValue::operator==(pLVar13,(L2CValue *)&LStack_a8);
      if ((uVar14 & 1) == 0) {
  LAB_710001ced4:
        pLVar13 = &LStack_a8;
        goto LAB_710001ced8;
      }
      pIVar16 = (Item *)lib::L2CValue::as_pointer(&LStack_98);
      bVar1 = Item::is_had(pIVar16,false);
      lib::L2CValue::L2CValue(&LStack_b8,(bool)(bVar1 & 1));
      lib::L2CValue::L2CValue(&LStack_78,false);
      uVar14 = lib::L2CValue::operator==(&LStack_b8,(L2CValue *)&LStack_78);
      if ((uVar14 & 1) == 0) {
        goto LAB_710001ced4;
      }
      uVar6 = lib::L2CValue::as_integer(&LStack_88);
      pvVar15 = (void *)app::sv_battle_object::module_accessor(uVar6);
      if (pvVar15 == (void *)0x0) {
        lib::L2CValue::L2CValue(&LStack_d8,(L2CValue *)&LUA_SCRIPT_LINE_SYSTEM_POST);
      }
      else {
        lib::L2CValue::L2CValue(&LStack_d8,pvVar15);
      }
      pBVar17 = (BattleObjectModuleAccessor *)lib::L2CValue::as_pointer(&LStack_d8);
      iVar5 = StatusModule::status_kind(pBVar17);
      lib::L2CValue::L2CValue(&LStack_c8,iVar5);
      lib::L2CValue::L2CValue(&LStack_78,_ITEM_STATUS_KIND_STANDBY);
      uVar14 = lib::L2CValue::operator==(&LStack_c8,(L2CValue *)&LStack_78);
      if ((uVar14 & 1) == 0) {
        lib::L2CValue::L2CValue(&LStack_78,_FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST);
        iVar5 = lib::L2CValue::as_integer(&LStack_78);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor,iVar5);
        lib::L2CValue::L2CValue(return_value,1);
        return;
      }
    }
    else {
      lib::L2CValue::L2CValue(&LStack_78,0x50000000);
      lib::L2CValue::L2CValue(&LStack_a8,_FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
      iVar5 = lib::L2CValue::as_integer(&LStack_78);
      iVar7 = lib::L2CValue::as_integer(&LStack_a8);
      WorkModule::set_int(fighter.module_accessor,iVar5,iVar7);
      pLVar13 = &LStack_78;
  LAB_710001ced8:
    }
  LAB_710001ceec:*/
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return 0.into();
}



pub fn install() {
    Agent::new("link")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre) // Status script
        .install();
}

#![feature(ptr_sub_ptr)]
#![allow(warnings)]

use std::fs;
use std::fs::read_to_string;
use skyline::install_hook;
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;
use unity::prelude::Il2CppString;
use engage::gamedata::item::ItemData;
use skyline::patching::Patch;
use engage::gamedata::PersonData;
use engage::gamedata::dispos::DisposData;
use engage::force::ForceType;
use engage::gameuserdata::GameUserData;
use miniserde::Serialize;
use miniserde::Deserialize;
use miniserde::json;
use lazy_static::lazy_static;
use std::str::FromStr;
use engage::gamedata::skill::SkillData;
use engage::mess::Mess;
use engage::proc::ProcInst;
use unity::prelude::Il2CppObject;
use engage::gamemessage::GameMessage;
use unity::system::string::SystemString;
use engage::gamedata::Gamedata;
use unity::prelude::OptionalMethod;
use engage::script::ScriptUtil;
use engage::proc::Bindable;
use engage::proc::ProcInstFields;
use engage::menu::BasicMenu;

static mut LearnList : Container = Container {vals: Vec::new()};

#[derive(Debug, Serialize, Deserialize)]
struct Learnset {
    pid: String,
    english: String,
    skills: Vec<(i32, String)>, // [(level, SID)]
}

#[derive(Debug, Serialize, Deserialize)]
struct Container{
    vals: Vec<Learnset>
}

#[unity::hook("App", "Unit", "GetCapabilityGrow")]
pub fn GetCapabilityGrow(this: &Unit, index: i32, auto_grow: bool , _method_info : u64) -> i32
{
    // auto_grow is false when it's the full rate (unit + job)
    // true when just unit (verify this)
    let mut val : i32 = call_original!(this, index, auto_grow, _method_info);

    // adjust display value for growth display mods
    if !auto_grow && this.has_sid("SID_一攫千金".into())
    {
        let jd: &JobData = this.get_job();
        if index == 6
        {
            val += jd.get_diff_grow()[1] as i32;

        }
        else if index == 1
        {
            val -= jd.get_diff_grow()[1] as i32;
        }
    }
    
    return val.into();
}

#[unity::hook("App", "Unit", "LevelUp")]
pub fn LevelUp(this: &Unit, abort: i32, _method_info : u64)
{unsafe{
    

    // normal stuff / job growth skill
    EditJobGrowths(this);
    call_original!(this, abort, _method_info);
    ResetJobGrowths(this);

    // maybe learn new skill
    // CheckForNewSkills(this, _method_info);
}}

pub fn EditJobGrowths(this: &Unit)
{
    // adjust values
    let act = this.has_sid("SID_一攫千金".into());
    if act
    {    
        let jd: &JobData = this.get_job();
        let grows = jd.get_diff_grow();
        let change_val = grows[1];
        grows[6] += change_val;
        grows[1] = 0;

        jd.set_diff_grow(grows);
    }
}

pub fn ResetJobGrowths(this: &Unit)
{
    // return values
    let act = this.has_sid("SID_一攫千金".into());
    if act
    {
        let jd: &JobData = this.get_job();
        let grows = jd.get_diff_grow();
        let change_val = grows[1];
        grows[6] -= change_val;
        grows[1] = change_val;

        jd.set_diff_grow(grows);
    }
}


#[skyline::from_offset(0x1A52530)]
pub fn get_unit_force(this: &Unit, _method_info: u64) -> i32;

#[unity::from_offset("App", "Unit", "AddToEquipSkillPool")]
pub fn AddToEquipSkillPool(this: &Unit, sid: &Il2CppString, _method_info: u64);

#[unity::from_offset("App", "Unit", "IsExistInEquipSkillPool")]
pub fn IsExistInEquipSkillPool(this: &Unit, sid: &Il2CppString, _method_info: u64) -> bool;

#[unity::from_offset("App", "Unit", "IsInheritanceEnable")]
pub fn IsInheritanceEnable(this: &Unit, skill: &SkillData, _method_info: u64) -> bool;


#[unity::class("App", "LevelUpSequnece")]
pub struct LevelUpSequnece {
    pub proc : ProcInstFields,
    pub pad1 : u64,
    pub pad2 : u64,
    pub unit : &'static Unit
}

impl Bindable for LevelUpSequnece {}

impl AsMut<ProcInstFields> for LevelUpSequnece {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}


#[unity::hook("App", "Unit", "CreateImpl2")]
pub fn RecruitUnit(this: &mut Option<Unit>, _method_info: u64)
{unsafe{
    call_original!(this, _method_info);

    if let Some(younit) = this
    {
        let force = get_unit_force(younit, _method_info);
        if force == ForceType::Player as i32 || force == ForceType::Ally as i32
        {
            // 0 = main menu
            // 3 = turn 1
            // 5 = post map
            // 7 = load map
            let seq = GameUserData::get_sequence();
            if seq == 3 || seq == 5 || seq == 7
            {
                if younit.person.get_asset_force() != ForceType::Enemy as i32
                {
                    //CheckForNewSkills(&this.as_ref().unwrap(), _method_info); //not sure this will work idk
                }
            }
        }
    }
}}


#[unity::hook("App", "LevelUpSequnece", "LearnJobSkill")]
pub fn LevelUpSeq(this: &mut Il2CppObject<ProcInst>, _method_info: u64)
{
    call_original!(this, _method_info);

    let seq = this.cast_mut::<LevelUpSequnece>();

    CheckForNewSkills(seq.unit, this, _method_info);
}

pub fn CheckForNewSkills(unit: &Unit, proc: &ProcInst, _method_info: u64)
{unsafe{

    let name = unit.get_pid();
    for x in LearnList.vals.iter_mut()
    {
        if x.pid == name.get_string().unwrap()
        {
            for y in &x.skills
            {
                if y.0 <= (unit.level + unit.internal_level as u8).into()
                {
                    if !IsExistInEquipSkillPool(unit, y.1.clone().into(), _method_info)
                    {
                        skyline::error::show_error(
                            69,
                            "test0.\n\0",
                            "0"
                        );
                        AddToEquipSkillPool(unit, Il2CppString::new(y.1.clone()), _method_info);
                        ShowSkillPopup(y.1.clone().into(), proc); // not skill data just a SID rn
                    }
                }
            }
        }
    }
}}

pub fn ShowSkillPopup(string: &Il2CppString, proc: &ProcInst)
{
    skyline::error::show_error(
        69,
        "test1.\n\0",
        "1"
    );
    if let Some(skill) = SkillData::get(&string.to_string()){
        // let proc: &'static ProcInst = ScriptUtil::get_sequence();
        let sid_substring: &str = &skill.sid.to_string()[4..];
        // let sid_substring: &str = "RefinedTaste";
        let tag: &Il2CppObject<SystemString> = Mess::create_sprite_tag(1, sid_substring.into());
        Mess::set_argument(0, tag);
        let skill_name: &mut Il2CppObject<SystemString> = Mess::get(skill.name.unwrap());
        Mess::set_argument(1,skill_name.to_string());
        let message: &'static mut Il2CppObject<SystemString> = Mess::get("MID_Hub_Inheritance_Skill_Finish");


        let err_msg = format!(
            "{}\n{}\n{}\0",
            skill_name,
            message,
            sid_substring
        );
        skyline::error::show_error(
            69,
            "Inherit new skill.\n\0",
            err_msg.as_str(),
        );
        GameMessage::create_key_wait(proc,message.to_string());
    }
}

pub fn LoadSkillJson()
{unsafe{
    let paths = fs::read_dir("sd:/engage/mods/PersonalSkillGrowth/json").unwrap();

    for path in paths
    {
        let p: &str = &read_to_string(path.unwrap().path()).unwrap();
        LearnList = json::from_str(p).expect("reason");
    }
}
}


#[skyline::main(name = "PersonalSkillsEnhanced")]
pub fn main() {
    println!("Personal Skills Enhanced loaded");

    LoadSkillJson();

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    
    install_hook!(LevelUp);
    install_hook!(GetCapabilityGrow);
    install_hook!(RecruitUnit);
    install_hook!(LevelUpSeq);
    

    // Patch these address to nop parts of functions which are called if SP Cost of a skill = 0
    // Credit to @BadAtGames26
    let addresses = [0x01a35fa4, 0x01a36f34, 0x01a36588, 0x01a38b68, 0x01a38b68, 0x01a35ec8, 0x01a391e8];
    for address in addresses {
        let patch = Patch::in_text(address).nop();
        if patch.is_ok() {
            patch.unwrap();
            println!("Patched address {:x} with NOP", address);
        }
    }
}

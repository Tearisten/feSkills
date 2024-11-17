#![feature(ptr_sub_ptr)]

use skyline::install_hook;
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;

#[unity::hook("App", "Unit", "GetCapabilityGrow")]
pub fn unit_getcapabilitygrow(this: &Unit, index: i32, auto_grow: bool , _method_info : u64) -> i32
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
pub fn unit_levelup(this: &Unit, abort: i32, _method_info : u64)
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

    call_original!(this, abort, _method_info);

    // return values
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

#[skyline::main(name = "convert")]
pub fn main() {
    println!("convert plugin loaded");

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
    
    install_hook!(unit_levelup);
    install_hook!(unit_getcapabilitygrow);
}

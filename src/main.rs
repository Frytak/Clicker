// Declaring modules...
mod raw_input;
mod message_handling;

use std::mem::size_of;

use windows::Win32::{UI::{Input::{
    KeyboardAndMouse::{INPUT, SendInput, INPUT_TYPE, INPUT_0, MOUSEINPUT, MOUSE_EVENT_FLAGS, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP},
    RID_DEVICE_INFO,
    RID_DEVICE_INFO_TYPE,
    RID_DEVICE_INFO_0
}, WindowsAndMessaging::GetMessageA}, Foundation::HWND};

use crate::{message_handling::{register_window_class, get_window_class, create_window}, raw_input::{register_raw_input, RAW_INPUT_KEYBOARD, get_raw_input_device_list, get_raw_input_device_info, get_raw_input_data}};

pub const MOUSE_CLICK: INPUT = INPUT {
    r#type: INPUT_TYPE(0),
    Anonymous: INPUT_0 {
        mi: MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: MOUSE_EVENT_FLAGS(MOUSEEVENTF_LEFTDOWN.0 | MOUSEEVENTF_LEFTUP.0),
            time: 0,
            dwExtraInfo: 0
        }
    }
};

fn main() -> Result<(), String> {

    // Create a window
    let window_class = get_window_class()?;
    let register_atom = register_window_class(&window_class)?;
    create_window(&window_class)?;

    // Check raw input
    register_raw_input(&RAW_INPUT_KEYBOARD)?;
    let device_list = get_raw_input_device_list()?;

    println!("====================================");
    println!("     Raw input devices status       ");
    println!("====================================");
    println!("Device count: {}", device_list.len());
    println!("Device list:");
    for (i, device) in device_list.iter().enumerate() {
        println!("\t{}: Handle({}), Type({})", i, device.hDevice.0, device.dwType.0);
    }



    for device in device_list {
        let device_info = get_raw_input_device_info(device.hDevice)?;


        println!("====================================");
        println!("      Raw input devices info        ");
        println!("====================================");
        println!("Device handle: {}", device.hDevice.0);

        match device_info.dwType {
            RID_DEVICE_INFO_TYPE(0) => unsafe {
                println!("\tId: {}", device_info.Anonymous.mouse.dwId);
                println!("\tNumber of buttons: {}", device_info.Anonymous.mouse.dwNumberOfButtons);
                println!("\tSample rate: {}", device_info.Anonymous.mouse.dwSampleRate);
                println!("\tHas horizontal wheel: {}", device_info.Anonymous.mouse.fHasHorizontalWheel.0);
            },
            RID_DEVICE_INFO_TYPE(1) => unsafe {
                println!("\tType: {}", device_info.Anonymous.keyboard.dwType);
                println!("\tSub-type: {}", device_info.Anonymous.keyboard.dwSubType);
                println!("\tMode: {}", device_info.Anonymous.keyboard.dwKeyboardMode);
                println!("\tNumber of function keys: {}", device_info.Anonymous.keyboard.dwNumberOfFunctionKeys);
                println!("\tNumber of indicators: {}", device_info.Anonymous.keyboard.dwNumberOfIndicators);
                println!("\tNumber of total keys: {}", device_info.Anonymous.keyboard.dwNumberOfKeysTotal);
            },
            RID_DEVICE_INFO_TYPE(2) => unsafe {
                println!("\tVendor ID: {}", device_info.Anonymous.hid.dwVendorId);
                println!("\tProduct ID: {}", device_info.Anonymous.hid.dwProductId);
                println!("\tVersion: {}", device_info.Anonymous.hid.dwVersionNumber);
                println!("\tUsage page: {}", device_info.Anonymous.hid.usUsagePage);
                println!("\tUsage: {}", device_info.Anonymous.hid.usUsage);
            },
            _ => { panic!("Impossible or yet unimplemented type of device."); }
        }
    }



    println!("Simulating mouse click...");
    std::thread::sleep(std::time::Duration::from_secs(2));
 
    unsafe {
        for _ in 0..1 {
            let error = SendInput(&[MOUSE_CLICK], size_of::<INPUT>() as i32);
            println!("{:?}", error);
        }
    }

    Ok(())
}

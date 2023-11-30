use std::{ptr::null_mut, mem::{size_of, self}, ffi::c_void};

use windows::Win32::{UI::{WindowsAndMessaging::{SetWindowsHookExA, WH_MOUSE, WH_KEYBOARD, WH_MOUSE_LL, GetMessageA, WINDOWS_HOOK_ID}, Input::{KeyboardAndMouse::{INPUT, INPUT_0, MOUSEINPUT, INPUT_TYPE, MOUSE_EVENT_FLAGS, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SendInput}, RAWINPUTDEVICE, RIDEV_NOLEGACY, RegisterRawInputDevices, GetRawInputDeviceList, RAWINPUTDEVICELIST, GetRawInputDeviceInfoA, RIDI_DEVICEINFO, RID_DEVICE_INFO, RID_DEVICE_INFO_TYPE, RID_DEVICE_INFO_0, RIDEV_REMOVE}}, Foundation::{WPARAM, LRESULT, LPARAM, HINSTANCE, HWND, GetLastError}, Devices::HumanInterfaceDevice::{HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_KEYBOARD, HID_USAGE_GENERIC_MOUSE}};

/// Array of input devices to register
///
/// This one only registers keyboards
const RAWINPUTDEVICES: [RAWINPUTDEVICE; 1] = [
    RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_KEYBOARD,
        dwFlags: RIDEV_NOLEGACY,
        hwndTarget: HWND(0)
    }
];

const MOUSE_CLICK: INPUT = INPUT {
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
    let mut device_list: Vec<RAWINPUTDEVICELIST>;
    let mut device_count: u32 = 0;

    unsafe {
        RegisterRawInputDevices(&RAWINPUTDEVICES, size_of::<RAWINPUTDEVICE>() as u32).map_err(|err| {
            format!("Couldn't register for raw input. {:?}", err)
        })?;

        // Get the amount of raw input devices and allocate an apropariate space
        if GetRawInputDeviceList(None, &mut device_count, size_of::<RAWINPUTDEVICELIST>() as u32) == u32::MAX {
            return Err(format!("Couldn't get the amount of raw input devices. {:?}", GetLastError().expect_err("No error found while getting the amount of raw input devices.")));
        }

        device_list = vec![RAWINPUTDEVICELIST::default(); device_count as usize];

        // Assign raw input devices into the array
        if GetRawInputDeviceList(Some(&mut device_list[0]), &mut device_count, size_of::<RAWINPUTDEVICELIST>() as u32) == u32::MAX {
            return Err(format!("Couldn't get the list of raw input devices. {:?}", GetLastError().expect_err("No error found while getting the list of raw input devices.")));
        }
    }

    println!("====================================");
    println!("     Raw input devices status       ");
    println!("====================================");
    println!("Device count: {}", device_count);
    println!("Device list:");
    for (i, device) in device_list.iter().enumerate() {
        println!("\t{}: Handle({}), Type({})", i, device.hDevice.0, device.dwType.0);
    }



    for device in device_list {
        let mut device_info_size: u32 = 0;
        let mut device_info = RID_DEVICE_INFO {
            cbSize: size_of::<RID_DEVICE_INFO>() as u32,
            dwType: RID_DEVICE_INFO_TYPE::default(),
            Anonymous: RID_DEVICE_INFO_0::default()
        };

        unsafe {
            let device_info_ptr: *mut c_void = (&mut device_info as *mut _) as *mut c_void;

            // Get required size... For no reason as I use RIDI_DEVICEINFO, I already know it but
            // Windows still requires it! Or else it errors that it doesn't have enough space for the
            // data even though it does.
            if GetRawInputDeviceInfoA(device.hDevice, RIDI_DEVICEINFO, None, &mut device_info_size) == u32::MAX {
                return Err(format!("Couldn't get required size for device with handle {}. {:?}", device.hDevice.0, GetLastError().expect_err("No error found while getting required size for device.")));
            }

            if GetRawInputDeviceInfoA(device.hDevice, RIDI_DEVICEINFO, Some(device_info_ptr), &mut device_info_size) == u32::MAX {
                return Err(format!("Couldn't get more information on device with handle {}. {:?}", device.hDevice.0, GetLastError().expect_err("No error found while getting more information on device.")));
            }
        }

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

    unsafe {
        GetMessageA(null_mut(), HWND::default(), 0, 0);
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

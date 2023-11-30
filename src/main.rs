use std::{ptr::null_mut, mem::{size_of, self}, ffi::c_void};

use windows::Win32::{UI::{WindowsAndMessaging::{SetWindowsHookExA, WH_MOUSE, WH_KEYBOARD, WH_MOUSE_LL, GetMessageA, WINDOWS_HOOK_ID}, Input::{KeyboardAndMouse::{INPUT, INPUT_0, MOUSEINPUT, INPUT_TYPE, MOUSE_EVENT_FLAGS, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SendInput}, RAWINPUTDEVICE, RIDEV_NOLEGACY, RegisterRawInputDevices, GetRawInputDeviceList, RAWINPUTDEVICELIST, GetRawInputDeviceInfoA, RIDI_DEVICEINFO, RID_DEVICE_INFO, RID_DEVICE_INFO_TYPE, RID_DEVICE_INFO_0}}, Foundation::{WPARAM, LRESULT, LPARAM, HINSTANCE, HWND, GetLastError}, Devices::HumanInterfaceDevice::{HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_KEYBOARD}};

const RAWINPUTDEVICES: [RAWINPUTDEVICE; 1] = [RAWINPUTDEVICE {usUsagePage: HID_USAGE_PAGE_GENERIC, usUsage: HID_USAGE_GENERIC_KEYBOARD, dwFlags: RIDEV_NOLEGACY, hwndTarget: HWND(0)}];
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

// unsafe fn callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {}
unsafe extern "system" fn callback(code: i32, param: WPARAM, lpdata: LPARAM) -> LRESULT {
    println!("F");
    //CallNextHookEx(, code, param, lpdata)
    LRESULT(0)
}

fn main() {
    unsafe {
        let mut device_list = [RAWINPUTDEVICELIST::default(); 8];
        let mut device_count: u32 = 0;
        let mut device_info = RID_DEVICE_INFO::default();
        let mut device_info_size: u32 = size_of::<RID_DEVICE_INFO>() as u32;
        let device_info_ptr: *mut c_void = (&mut device_info as *mut _) as *mut c_void;

        device_info.cbSize = size_of::<RID_DEVICE_INFO>() as u32;
        dbg!(device_info.cbSize, size_of::<RID_DEVICE_INFO>());

        RegisterRawInputDevices(&RAWINPUTDEVICES, size_of::<RAWINPUTDEVICE>() as u32).unwrap();
        GetRawInputDeviceList(None, &mut device_count, size_of::<RAWINPUTDEVICELIST>() as u32);
        // Alloc device_count
        GetRawInputDeviceList(Some(&mut device_list[0]), &mut device_count, size_of::<RAWINPUTDEVICELIST>() as u32);
        dbg!(device_count, device_list);
        dbg!(GetRawInputDeviceInfoA(device_list[0].hDevice, RIDI_DEVICEINFO, Some(device_info_ptr), &mut device_info_size));
        dbg!(device_list[0], device_info.dwType);
        //SetWindowsHookExA(WINDOWS_HOOK_ID(13), Some(callback), HINSTANCE::default(), 0).unwrap();
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
}

use windows::Win32::{UI::Input::{RAWINPUTDEVICE, RIDEV_NOLEGACY, KeyboardAndMouse::{INPUT, INPUT_TYPE, INPUT_0, MOUSEINPUT, MOUSE_EVENT_FLAGS, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP}}, Devices::HumanInterfaceDevice::{HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_KEYBOARD}, Foundation::HWND};

/// Array of input devices to register
///
/// This one only registers keyboards
pub const RAWINPUTDEVICES: [RAWINPUTDEVICE; 1] = [
    RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_KEYBOARD,
        dwFlags: RIDEV_NOLEGACY, hwndTarget: HWND(0) }
];

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

pub 
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

use std::{mem::size_of, ffi::c_void, fmt::Debug};

use windows::Win32::{
    UI::Input::{
        RAWINPUTDEVICE,
        RID_DEVICE_INFO,
        HRAWINPUT,
        RAWINPUTHEADER,
        RAWINPUTDEVICELIST,
        RegisterRawInputDevices,
        GetRawInputDeviceList,
        GetRawInputDeviceInfoA,
        GetRawInputData,
        RIDEV_NOLEGACY,
        RIDI_DEVICEINFO,
        RID_HEADER,
        RID_INPUT, RAWKEYBOARD, RAWMOUSE, RAWHID, RID_DEVICE_INFO_TYPE
    },
    Devices::HumanInterfaceDevice::{HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_KEYBOARD},
    Foundation::{HWND, GetLastError, HANDLE}
};

/// Array of input devices to register
///
/// This one only registers keyboards
pub const RAW_INPUT_KEYBOARD: [RAWINPUTDEVICE; 1] = [
    RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_KEYBOARD,
        dwFlags: RIDEV_NOLEGACY, hwndTarget: HWND(0) }
];

pub fn register_raw_input(raw_input_devices: &[RAWINPUTDEVICE]) -> Result<(), String> {
    unsafe {
        RegisterRawInputDevices(raw_input_devices, size_of::<RAWINPUTDEVICE>() as u32).map_err(|err| {
            format!("Couldn't register for raw input. Error: `{:?}`", err)
        })?;
    }

    Ok(())
}

pub fn get_raw_input_device_list() -> Result<Vec<RAWINPUTDEVICELIST>, String> {
    let mut device_list: Vec<RAWINPUTDEVICELIST>;
    let mut device_count = 0;
    unsafe {

        // Get the amount of raw input devices and allocate an apropariate space
        if GetRawInputDeviceList(None, &mut device_count, size_of::<RAWINPUTDEVICELIST>() as u32) == u32::MAX {
            return Err(
                format!("Couldn't get the amount of raw input devices. {:?}",
                GetLastError().expect_err("No error found while getting the amount of raw input devices."))
            );
        }

        device_list = vec![RAWINPUTDEVICELIST::default(); device_count as usize];

        // Assign raw input devices into the array
        if GetRawInputDeviceList(Some(&mut device_list[0]), &mut device_count, size_of::<RAWINPUTDEVICELIST>() as u32) == u32::MAX {
            return Err(
                format!("Couldn't get the list of raw input devices. {:?}",
                GetLastError().expect_err("No error found while getting the list of raw input devices."))
            );
        }
    }

    Ok(device_list)
}

pub fn get_raw_input_device_info(device_handle: HANDLE) -> Result<RID_DEVICE_INFO, String> {
    let mut device_info: RID_DEVICE_INFO = RID_DEVICE_INFO::default();
    let device_info_ptr: *mut c_void;
    let mut device_info_size: u32 = 0;

    unsafe {
        device_info_ptr = (&mut device_info as *mut _) as *mut c_void;
        // Get required size... For no reason as I use RIDI_DEVICEINFO, I already know it but
        // Windows still requires it! Or else it errors that it doesn't have enough space for the
        // data even though it does.
        if GetRawInputDeviceInfoA(device_handle, RIDI_DEVICEINFO, None, &mut device_info_size) == u32::MAX {
            return Err(
                format!("Couldn't get required size for device with handle {}. {:?}",
                device_handle.0, GetLastError().expect_err("No error found while getting required size for device."))
            );
        }

        if GetRawInputDeviceInfoA(device_handle, RIDI_DEVICEINFO, Some(device_info_ptr), &mut device_info_size) == u32::MAX {
            return Err(
                format!("Couldn't get more information on device with handle {}. {:?}",
                device_handle.0, GetLastError().expect_err("No error found while getting more information on device."))
            );
        }

        Ok(device_info)
    }
}

union UnsafeRawInputData {
    Keyboard: RAWKEYBOARD,
    Mouse: RAWMOUSE,
    HID: RAWHID
}

enum RawInputData {
    Keyboard(RAWKEYBOARD),
    Mouse(RAWMOUSE),
    HID(RAWHID)
}

impl Debug for RawInputData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Keyboard(keyboard) => { &format!("RAWKEYBOARD {{ MakeCode: {}, Flags: {}, Reserved: {}, VKey: {}, Message: {}, ExtraInformation: {} }}",
            keyboard.MakeCode, keyboard.Flags, keyboard.Reserved, keyboard.VKey, keyboard.Message, keyboard.ExtraInformation) }
            Self::Mouse(mouse) => { &format!("RAWMOUSE {{ usFlags: {}, Anonymous: {{ ulButtons: {}, Anonymous: {{ usButtonData: {}, usButtonFlags: {} }} }}, ulRawButtons: {}, lLastX: {}, lLastY: {}, ulExtraInformation: {} }}", mouse.usFlags, mouse.Anonymous.ulButtons, mouse.Anonymous.Anonymous.usButtonData, mouse.Anonymous.Anonymous.usButtonFlags, mouse.ulRawButtons, mouse.lLastX, mouse.lLastY, mouse.ulExtraInformation) }
            Self::HID(hid) => { &format!("LOL") }
        });

        Ok(())
    }
}

pub fn get_raw_input_data(data_handle: &HRAWINPUT) -> Result<RawInputData, String> {
    // Get the header
    let mut device_header: RAWINPUTHEADER = RAWINPUTHEADER::default();

    unsafe {
        let device_header_ptr = (&mut device_header as *mut _) as *mut c_void;
        GetRawInputData(*data_handle, RID_HEADER, Some(device_header_ptr), std::ptr::null_mut(), size_of::<RAWINPUTHEADER>() as u32);
    }

    // Check the type of device data
    let device_info = get_raw_input_device_info(device_header.hDevice)?;

    // Get the device data
    let mut device_data: UnsafeRawInputData; 
    unsafe {
        let mut data_size = 0;
        let device_data_ptr = (&mut device_data as *mut _) as *mut c_void;
        if GetRawInputData(*data_handle, RID_INPUT, Some(device_data_ptr), &mut data_size as *mut _, size_of::<RAWINPUTHEADER>() as u32) == u32::MAX {
            return Err(
                format!("Couldn't get data of device with handle {}. {:?}",
                device_header.hDevice.0, GetLastError().expect_err("No error found while getting more information on device."))
            );
        }
    }

    match device_info.dwType {
        RID_DEVICE_INFO_TYPE(0) => unsafe { Ok(RawInputData::Mouse(device_data.Mouse)) },
        RID_DEVICE_INFO_TYPE(1) => unsafe { Ok(RawInputData::Keyboard(device_data.Keyboard)) },
        RID_DEVICE_INFO_TYPE(2) => unsafe { Ok(RawInputData::HID(device_data.HID)) }
    }
}

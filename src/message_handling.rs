use windows::{
    Win32::{
        UI::WindowsAndMessaging::{ WNDCLASSA, WNDCLASS_STYLES, LoadCursorW, IDC_ARROW, DefWindowProcA, RegisterClassA, CreateWindowExA, WINDOW_EX_STYLE, WINDOW_STYLE, HWND_MESSAGE, HMENU, WM_INPUT },
        Foundation::{ HWND, HINSTANCE, LRESULT, GetLastError, WPARAM, LPARAM },
        Graphics::Gdi::{ HBRUSH, COLOR_WINDOW },
        System::LibraryLoader::GetModuleHandleA,
    },
    core::PCSTR
};

pub const APP_NAME: &'static str = "FrytaksClicker";
const APP_NAME_WIN: [u8; 15] = *b"FrytaksClicker\0";

/// A window class specifically for this app
///
/// Couldn't be defined as a `const` as it uses non-constant function calls
pub unsafe fn get_window_class() -> Result<WNDCLASSA, String> {
    Ok(WNDCLASSA {
        style: WNDCLASS_STYLES::default(),
        lpfnWndProc: Some(window_handle_message),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: HINSTANCE(unsafe{GetModuleHandleA(PCSTR::null()).unwrap().0} as isize),
        hIcon: windows::Win32::UI::WindowsAndMessaging::HICON::default(),
        hCursor: unsafe{LoadCursorW(HINSTANCE::default(), IDC_ARROW).unwrap()},
        hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize),
        lpszMenuName: PCSTR::null(),
        lpszClassName: PCSTR::from_raw(&APP_NAME_WIN[0] as *const _),
    })
}

/// Function handling incoming messages in a queue.
unsafe extern "system" fn window_handle_message(handle: HWND, message: u32, additional_w: WPARAM, additional_l: LPARAM) -> LRESULT {
    println!("We've got some messages boys!");

    match message {
        WM_INPUT => {
            println!("AND IT'S A RAW INPUT BABEEEEE!");
        }
        _ => { return DefWindowProcA(handle, message, additional_w, additional_l); }
    }

    LRESULT::default()
}

pub fn register_window_class(window_class: &WNDCLASSA) -> Result<u16, String> {
    // Register the window class
    let register_atom = unsafe { RegisterClassA(window_class as *const _) };

    // Handle window class registration error
    if register_atom == 0 {
        return Err(
            format!("Registering a window class was unsuccesful. Error: `{:?}`.",
            unsafe { GetLastError().expect_err("No error found while registering a class even though it errored.") } )
        );
    }

    Ok(register_atom)
}

/// Create an invisible window for getting messages
///
/// I need `WM_INPUT` messages to be specific
pub fn create_window(window_class: &WNDCLASSA) -> Result<(), String> {
    let window_handle: HWND = unsafe { CreateWindowExA(
        WINDOW_EX_STYLE::default(),
        window_class.lpszClassName,
        PCSTR::null(),
        WINDOW_STYLE::default(),
        0,
        0,
        0,
        0,
        HWND_MESSAGE,
        HMENU::default(),
        window_class.hInstance,
        None
    ) };

    // Handle window creation error
    if window_handle.0 == 0 {
        return Err(
            format!("Window creation was unsuccesful. Error: {:?}",
            unsafe { GetLastError().expect_err("No error found for creating a window") } )
        );
    }

    Ok(())
}

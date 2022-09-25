// pub fn key_is_pressed(ks:KeyCode) -> bool{
//     return unsafe{ ffi::GetAsyncKeyState(ks) & 0x8000 } as bool;
// }


#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
#[derive(PartialEq)]
pub enum KeyCode 
{
    KEY_UP       = VK_UP.0 as isize,
    KEY_DOWN     = VK_DOWN.0 as isize,
    KEY_LEFT     = VK_LEFT.0 as isize,
    KEY_RIGHT    = VK_RIGHT.0 as isize,
    KEY_SHIFT_L  = VK_LSHIFT.0 as isize,
}

pub fn key_is_pressed(ks:&KeyCode) -> bool{
    let vkey:i32 = *ks as i32;
    return unsafe {GetAsyncKeyState(vkey) != 0} ;
}

#[cfg(target_os = "linux")]
pub fn key_is_pressed(ks:KeyCode) -> bool{
    return false;
}

#[cfg(target_os = "macos")]
pub fn key_is_pressed(ks:KeyCode) -> bool{
    return false;
}

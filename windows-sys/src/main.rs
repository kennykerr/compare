use windows_sys::{Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), BOOL(1), BOOL(0), PWSTR(std::ptr::null_mut()));
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(HWND(0), PSTR(b"Text\0".as_ptr() as _), PSTR(b"Caption\0".as_ptr() as _), MB_OK);
    }
}

//! Giving memory back to the system, as Edge does for tabs in the background.

/// Return freed heap memory to the system (after an app closes).
pub fn compact() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Memory::{GetProcessHeap, HeapCompact};
        // SAFETY: the process's own default heap, a handle that is always valid.
        #[allow(unsafe_code)]
        unsafe {
            let heap = GetProcessHeap();
            if !heap.is_null() {
                HeapCompact(heap, 0);
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        // SAFETY: glibc's malloc_trim has no preconditions.
        #[allow(unsafe_code)]
        unsafe {
            libc::malloc_trim(0);
        }
    }
}

/// Krasis is minimized or in the background: page out what it isn't using, so it holds almost
/// no memory until it is used again (pages come back from RAM as they are touched).
pub fn trim_working_set() {
    compact();
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::{ProcessStatus::K32EmptyWorkingSet, Threading::GetCurrentProcess};
        // SAFETY: this process's pseudo-handle, always valid.
        #[allow(unsafe_code)]
        unsafe {
            K32EmptyWorkingSet(GetCurrentProcess());
        }
    }
}

/// Let the running Krasis (process `pid`) come to the front when a later start hands it files:
/// Windows lets only the process the user is working in bring a window forward.
pub fn allow_foreground(pid: u32) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow;
        // SAFETY: a plain call with a process id; a stale id only makes it fail.
        #[allow(unsafe_code)]
        unsafe {
            AllowSetForegroundWindow(pid);
        }
    }
    #[cfg(not(windows))]
    let _ = pid;
}

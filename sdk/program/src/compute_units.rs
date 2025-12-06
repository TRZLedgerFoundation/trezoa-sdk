/// Return the remaining compute units the program may consume
#[inline]
pub fn sol_remaining_compute_units() -> u64 {
    #[cfg(target_os = "trezoa")]
    unsafe {
        crate::syscalls::sol_remaining_compute_units()
    }

    #[cfg(not(target_os = "trezoa"))]
    {
        crate::program_stubs::sol_remaining_compute_units()
    }
}

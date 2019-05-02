#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Receiver Interface
//!
//! Used by: stm32f7x5, stm32f7x6, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use stm32f7::peripherals::spdifrx::Instance;
pub use stm32f7::peripherals::spdifrx::{RegisterBlock, ResetValues};
pub use stm32f7::peripherals::spdifrx::{CR, CSR, DIR, DR, IFCR, IMR, SR};

/// Access functions for the SPDIFRX peripheral instance
pub mod SPDIFRX {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPDIFRX
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        IMR: 0x00000000,
        SR: 0x00000000,
        IFCR: 0x00000000,
        DR: 0x00000000,
        CSR: 0x00000000,
        DIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPDIFRX_TAKEN: bool = false;

    /// Safe access to SPDIFRX
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPDIFRX_TAKEN {
                None
            } else {
                SPDIFRX_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPDIFRX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPDIFRX_TAKEN && inst.addr == INSTANCE.addr {
                SPDIFRX_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPDIFRX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPDIFRX_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPDIFRX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPDIFRX: *const RegisterBlock = 0x40004000 as *const _;

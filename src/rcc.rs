use core::cmp;

use cast::u32;
use stm32l4::stm32l4x2::{rcc, RCC};

use flash::ACR;
use time::Hertz;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb1: AHB1 { _0: () },
            ahb2: AHB2 { _0: () },
            ahb3: AHB3 { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            cfgr: CFGR {
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// AMBA High-performance Bus (AHB1) registers
    pub ahb1: AHB1,
    /// AMBA High-performance Bus (AHB1) registers
    pub ahb2: AHB2,
    /// AMBA High-performance Bus (AHB1) registers
    pub ahb3: AHB3,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    pub cfgr: CFGR,
}

// AMBA High-performance Bus (AHB1) registers
pub struct AHB1 {
    _0: (),
}

impl AHB1 {
    pub(crate) fn enr(&mut self) -> &rcc::AHB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::AHB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1rstr }
    }
}

// AMBA High-performance Bus (AHB2) registers
pub struct AHB2 {
    _0: (),
}

impl AHB2 {
    pub(crate) fn enr(&mut self) -> &rcc::AHB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::AHB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2rstr }
    }
}

// AMBA High-performance Bus (AHB3) registers
pub struct AHB3 {
    _0: (),
}

impl AHB3 {
    pub(crate) fn enr(&mut self) -> &rcc::AHB3ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::AHB3RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3rstr }
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR1 {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1enr1 }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR1 {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1rstr1 }
    }
}

/// Advanced Peripheral Bus 2 (APB2) registers
pub struct APB2 {
    _0: (),
}

impl APB2 {
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2rstr }
    }
}

const HSI: u32 = 8_000_000; // Hz

pub struct CFGR {
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

impl CFGR {
    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk1<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk1 = Some(freq.into().0);
        self
    }

    pub fn pclk2<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk2 = Some(freq.into().0);
        self
    }

    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
        self
    }

    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        // TODO ADC & USB clocks

        let pllmul = (4 * self.sysclk.unwrap_or(HSI) + HSI) / HSI / 2;
        let pllmul = cmp::min(cmp::max(pllmul, 2), 16);
        let pllmul_bits = if pllmul == 2 {
            None
        } else {
            Some(pllmul as u8 - 2)
        };

        let sysclk = pllmul * HSI / 2;

        assert!(sysclk < 72_000_000);

        let hpre_bits = self.hclk
            .map(|hclk| match sysclk / hclk {
                0 => unreachable!(),
                1 => 0b0111,
                2 => 0b1000,
                3...5 => 0b1001,
                6...11 => 0b1010,
                12...39 => 0b1011,
                40...95 => 0b1100,
                96...191 => 0b1101,
                192...383 => 0b1110,
                _ => 0b1111,
            })
            .unwrap_or(0b0111);

        let hclk = sysclk / (1 << (hpre_bits - 0b0111));

        assert!(hclk < 72_000_000);

        let ppre1_bits = self.pclk1
            .map(|pclk1| match hclk / pclk1 {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre1 = 1 << (ppre1_bits - 0b011);
        let pclk1 = hclk / u32(ppre1);

        assert!(pclk1 <= 36_000_000);

        let ppre2_bits = self.pclk2
            .map(|pclk2| match hclk / pclk2 {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre2 = 1 << (ppre2_bits - 0b011);
        let pclk2 = hclk / u32(ppre2);

        assert!(pclk2 < 72_000_000);

        // adjust flash wait states
        unsafe {
            acr.acr().write(|w| {
                w.latency().bits(if sysclk <= 24_000_000 {
                    0b000
                } else if sysclk <= 48_000_000 {
                    0b001
                } else {
                    0b010
                })
            })
        }

        // TODO FIX
        let rcc = unsafe { &*RCC::ptr() };
        if let Some(pllmul_bits) = pllmul_bits {
            // use PLL as source

            // rcc.pllcfgr.write(|w| unsafe { w.plln().bits(pllmul_bits) });

            // // turn on pll
            // rcc.cr.write(|w| { 
            //     w.pllon().set_bit()
            // });
            // // wait till ready
            // while rcc.cr.read().pllrdy().bit_is_set() {}

            // /* 
            // Bits 3:2SWS[1:0]: System clock switch status
            // Set and cleared by hardware to indicate which clock source is used as system clock.
            // 00: MSI oscillator used as system clock
            // 01: HSI16 oscillator used as system clock10: HSE used as system clock
            // 11: PLL used as system clock
            // Bits 1:0
            // SW[1:0]: System clock switchSet and cleared by software to select system clock source (SYSCLK).
            // Configured by HW to force MSI oscillator selection when exiting Standby or Shutdown mode. 
            // Configured by HW to force MSI or HSI16 oscillator selection when exiting Stop mode or in case of failure of the HSE oscillator, depending on STOPWUCK value.00: MSI selected as system clock01: HSI16 selected as system clock10: HSE selected as system clock11: PLL selected as system clock
            //  */

            // let pll_switch_bits = 0b00000011;

            // rcc.cfgr.modify(|_, w| unsafe {
            //     w.ppre2()
            //         .bits(ppre2_bits)
            //         .ppre1()
            //         .bits(ppre1_bits)
            //         .hpre()
            //         .bits(hpre_bits)
            //         .sw() // finally switch to pll clock source
            //         .bits(pll_switch_bits)
            // });

            // // assert the pll bits are now set in the status register
            // assert!(rcc.cfgr.read().sws().bits() == pll_switch_bits);
        } else {
            //use HSI16 as source
            let hsi_switch_bits = 0b00000001;

            // turn on hsi
            rcc.cr.write(|w| { 
                w.hsion().set_bit()
            });

            // wait till ready
            while rcc.cr.read().hsirdy().bit_is_set() {}

            rcc.cfgr.write(|w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .bits(hsi_switch_bits)
            });

            assert!(rcc.cfgr.read().sws().bits() == hsi_switch_bits);
        }

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }

    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
embassy_hal_internal::peripherals_definition!(
    PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15, PB0, PB1,
    PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3,
    PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15, PD2, PF0, PF1, PF4, PF5, PF6,
    PF7, ADC1, CEC, CRC, DAC1, DBGMCU, DMA1, FLASH, I2C1, I2C2, IWDG, PWR, MCO, RCC, RTC, SPI1,
    SPI2, SYSCFG, TIM1, TIM14, TIM15, TIM16, TIM17, TIM2, TIM3, TIM6, TSC, UID, USART1, USART2,
    WWDG, EXTI0, EXTI1, EXTI2, EXTI3, EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11,
    EXTI12, EXTI13, EXTI14, EXTI15, DMA1_CH1, DMA1_CH2, DMA1_CH3, DMA1_CH4, DMA1_CH5
);
embassy_hal_internal::peripherals_struct!(
    PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15, PB0, PB1,
    PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3,
    PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15, PD2, PF0, PF1, PF4, PF5, PF6,
    PF7, ADC1, CEC, CRC, DAC1, DBGMCU, DMA1, FLASH, I2C1, I2C2, IWDG, PWR, MCO, RCC, RTC, SPI1,
    SPI2, SYSCFG, TIM1, TIM14, TIM15, TIM16, TIM17, TIM3, TIM6, TSC, UID, USART1, USART2, WWDG,
    EXTI0, EXTI1, EXTI2, EXTI3, EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11, EXTI12,
    EXTI13, EXTI14, EXTI15, DMA1_CH1, DMA1_CH2, DMA1_CH3, DMA1_CH4, DMA1_CH5
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD,
    RTC,
    FLASH,
    RCC,
    EXTI0_1,
    EXTI2_3,
    EXTI4_15,
    TSC,
    DMA1_CHANNEL1,
    DMA1_CHANNEL2_3,
    DMA1_CHANNEL4_5,
    ADC1_COMP,
    TIM1_BRK_UP_TRG_COM,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM6_DAC,
    TIM14,
    TIM15,
    TIM16,
    TIM17,
    I2C1,
    I2C2,
    SPI1,
    SPI2,
    USART1,
    USART2,
    CEC_CAN,
);
pub const MAX_ERASE_SIZE: usize = 1024u32 as usize;
pub mod flash_regions {
    pub const BANK1_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134217728u32,
        size: 65536u32,
        erase_size: 1024u32,
        write_size: 4u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::PeripheralRef<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region: Bank1Region<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(
            p: embassy_hal_internal::PeripheralRef<'d, crate::peripherals::FLASH>,
        ) -> Self {
            Self {
                bank1_region: Bank1Region(
                    &BANK1_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 1usize] = [&BANK1_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 9u8)),
            (6u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CEC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.cfgr3().read().cecsw() {
            crate::pac::rcc::vals::Cecsw::HSI_DIV_244 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi_div_244 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "HSI_DIV_244")
            },
            crate::pac::rcc::vals::Cecsw::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 30u8)),
            (7u8, 30u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CEC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (5u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 29u8)),
            (7u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DBGMCU {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DBGMCU" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 22u8)),
            (6u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DBGMCU {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (5u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FLASH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (5u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FLASH {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.cfgr3().read().i2c1sw() {
            crate::pac::rcc::vals::Icsw::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "HSI")
            },
            crate::pac::rcc::vals::Icsw::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "SYS")
            },
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 21u8)),
            (7u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 22u8)),
            (7u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 28u8)),
            (7u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 12u8)),
            (6u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 14u8)),
            (7u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 0u8)),
            (6u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 11u8)),
            (6u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM14 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 8u8)),
            (7u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM14 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM15 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 16u8)),
            (6u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM15 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM16 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 17u8)),
            (6u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM16 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM17 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 18u8)),
            (6u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM17 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 0u8)),
            (7u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 1u8)),
            (7u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 4u8)),
            (7u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TSC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TSC" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 24u8)),
            (5u8, 24u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TSC {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.cfgr3().read().usart1sw() {
            crate::pac::rcc::vals::Usart1sw::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "PCLK2")
            },
            crate::pac::rcc::vals::Usart1sw::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "SYS")
            },
            crate::pac::rcc::vals::Usart1sw::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "LSE")
            },
            crate::pac::rcc::vals::Usart1sw::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "HSI")
            },
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 14u8)),
            (6u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.cfgr3().read().usart2sw() {
            crate::pac::rcc::vals::Usartsw::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "PCLK1")
            },
            crate::pac::rcc::vals::Usartsw::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "SYS")
            },
            crate::pac::rcc::vals::Usartsw::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "LSE")
            },
            crate::pac::rcc::vals::Usartsw::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "HSI")
            },
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 17u8)),
            (7u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 11u8)),
            (7u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 0usize] = [];
pub mod mux {
    pub use crate::pac::rcc::vals::Cecsw;
    pub use crate::pac::rcc::vals::Icsw;
    pub use crate::pac::rcc::vals::Usart1sw;
    pub use crate::pac::rcc::vals::Usartsw;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub cecsw: Cecsw,
        pub i2c1sw: Icsw,
        pub usart1sw: Usart1sw,
        pub usart2sw: Usartsw,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.cfgr3().modify(|w| {
                w.set_cecsw(self.cecsw);
                w.set_i2c1sw(self.i2c1sw);
                w.set_usart1sw(self.usart1sw);
                w.set_usart2sw(self.usart2sw);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub hclk1: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub hsi_div_244: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_dma() {}
pub unsafe fn init_bdma() {
    crate::pac::RCC.ahbenr().modify(|w| w.set_dmaen(true));
}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.ahbenr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.ahbenr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.ahbenr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.ahbenr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.ahbenr().modify(|w| w.set_gpiofen(true));
}
impl_adc_pin!(ADC1, PA0, 0u8);
impl_adc_pin!(ADC1, PA1, 1u8);
impl_adc_pin!(ADC1, PA2, 2u8);
impl_adc_pin!(ADC1, PA3, 3u8);
impl_adc_pin!(ADC1, PA4, 4u8);
impl_adc_pin!(ADC1, PA5, 5u8);
impl_adc_pin!(ADC1, PA6, 6u8);
impl_adc_pin!(ADC1, PA7, 7u8);
impl_adc_pin!(ADC1, PB0, 8u8);
impl_adc_pin!(ADC1, PB1, 9u8);
impl_adc_pin!(ADC1, PC0, 10u8);
impl_adc_pin!(ADC1, PC1, 11u8);
impl_adc_pin!(ADC1, PC2, 12u8);
impl_adc_pin!(ADC1, PC3, 13u8);
impl_adc_pin!(ADC1, PC4, 14u8);
impl_adc_pin!(ADC1, PC5, 15u8);
impl_dac_pin!(DAC1, PA4, 1u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB6, 1u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB7, 1u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB8, 1u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB9, 1u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PB10, 1u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PB11, 1u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PF6, 0u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PF7, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO, PA8, 0u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PA15, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA15, 0u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PA4, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA4, 0u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PA5, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PA5, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PA6, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PA6, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PA7, 0u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PB3, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PB3, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PB4, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PB4, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PB5, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB12, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB13, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PB14, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PB15, 0u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM1, PA10, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM1, PA11, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM1, PA12, 2u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM1, PA6, 2u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM1, PA7, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM1, PA8, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM1, PA9, 2u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM1, PB0, 2u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM1, PB1, 2u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM1, PB12, 2u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM1, PB13, 2u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM1, PB14, 2u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM1, PB15, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM14, PA4, 4u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM14, PA7, 4u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM14, PB1, 0u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM15, PA2, 0u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM15, PA3, 0u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM15, PA9, 0u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM15, PB14, 1u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM15, PB15, 3u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM15, PB15, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM16, PA6, 5u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM16, PB5, 2u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM16, PB6, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM16, PB8, 2u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM17, PA10, 0u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM17, PA7, 5u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM17, PB7, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM17, PB9, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM2, PA0, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA0, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM2, PA1, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM2, PA15, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA15, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM2, PA2, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM2, PA3, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM2, PA5, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA5, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM2, PB10, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM2, PB11, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM2, PB3, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM3, PA6, 1u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM3, PA7, 1u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM3, PB0, 1u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM3, PB1, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM3, PB4, 1u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM3, PB5, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM3, PC6, 0u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM3, PC7, 0u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM3, PC8, 0u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM3, PC9, 0u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM3, PD2, 0u8);
pin_trait_impl!(crate::tsc::G1IO1Pin, TSC, PA0, 3u8);
pin_trait_impl!(crate::tsc::G1IO2Pin, TSC, PA1, 3u8);
pin_trait_impl!(crate::tsc::G4IO2Pin, TSC, PA10, 3u8);
pin_trait_impl!(crate::tsc::G4IO3Pin, TSC, PA11, 3u8);
pin_trait_impl!(crate::tsc::G4IO4Pin, TSC, PA12, 3u8);
pin_trait_impl!(crate::tsc::G1IO3Pin, TSC, PA2, 3u8);
pin_trait_impl!(crate::tsc::G1IO4Pin, TSC, PA3, 3u8);
pin_trait_impl!(crate::tsc::G2IO1Pin, TSC, PA4, 3u8);
pin_trait_impl!(crate::tsc::G2IO2Pin, TSC, PA5, 3u8);
pin_trait_impl!(crate::tsc::G2IO3Pin, TSC, PA6, 3u8);
pin_trait_impl!(crate::tsc::G2IO4Pin, TSC, PA7, 3u8);
pin_trait_impl!(crate::tsc::G4IO1Pin, TSC, PA9, 3u8);
pin_trait_impl!(crate::tsc::G3IO2Pin, TSC, PB0, 3u8);
pin_trait_impl!(crate::tsc::G3IO3Pin, TSC, PB1, 3u8);
pin_trait_impl!(crate::tsc::G6IO1Pin, TSC, PB11, 3u8);
pin_trait_impl!(crate::tsc::G6IO2Pin, TSC, PB12, 3u8);
pin_trait_impl!(crate::tsc::G6IO3Pin, TSC, PB13, 3u8);
pin_trait_impl!(crate::tsc::G6IO4Pin, TSC, PB14, 3u8);
pin_trait_impl!(crate::tsc::G3IO4Pin, TSC, PB2, 3u8);
pin_trait_impl!(crate::tsc::G5IO1Pin, TSC, PB3, 3u8);
pin_trait_impl!(crate::tsc::G5IO2Pin, TSC, PB4, 3u8);
pin_trait_impl!(crate::tsc::G5IO3Pin, TSC, PB6, 3u8);
pin_trait_impl!(crate::tsc::G5IO4Pin, TSC, PB7, 3u8);
pin_trait_impl!(crate::tsc::G3IO1Pin, TSC, PC5, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PA10, 1u8);
pin_trait_impl!(crate::usart::CtsPin, USART1, PA11, 1u8);
pin_trait_impl!(crate::usart::DePin, USART1, PA12, 1u8);
pin_trait_impl!(crate::usart::RtsPin, USART1, PA12, 1u8);
pin_trait_impl!(crate::usart::CkPin, USART1, PA8, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PA9, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PB6, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PB7, 0u8);
pin_trait_impl!(crate::usart::CtsPin, USART2, PA0, 1u8);
pin_trait_impl!(crate::usart::DePin, USART2, PA1, 1u8);
pin_trait_impl!(crate::usart::RtsPin, USART2, PA1, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PA14, 1u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PA15, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PA2, 1u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PA3, 1u8);
pin_trait_impl!(crate::usart::CkPin, USART2, PA4, 1u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH1, ());
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH2, ());
dma_trait_impl!(crate::dac::DacDma1, DAC1, DMA1_CH3, ());
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH2, ());
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH3, ());
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, ());
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH5, ());
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH2, ());
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH3, ());
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH4, ());
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH5, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM1, DMA1_CH2, ());
dma_trait_impl!(crate::timer::Ch2Dma, TIM1, DMA1_CH3, ());
dma_trait_impl!(crate::timer::Ch4Dma, TIM1, DMA1_CH4, ());
dma_trait_impl!(crate::timer::Ch3Dma, TIM1, DMA1_CH5, ());
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH5, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM15, DMA1_CH5, ());
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH5, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM16, DMA1_CH3, ());
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH3, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM16, DMA1_CH4, ());
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH4, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM17, DMA1_CH1, ());
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH1, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM17, DMA1_CH2, ());
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH2, ());
dma_trait_impl!(crate::timer::Ch3Dma, TIM2, DMA1_CH1, ());
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH2, ());
dma_trait_impl!(crate::timer::Ch2Dma, TIM2, DMA1_CH3, ());
dma_trait_impl!(crate::timer::Ch4Dma, TIM2, DMA1_CH4, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM2, DMA1_CH5, ());
dma_trait_impl!(crate::timer::Ch3Dma, TIM3, DMA1_CH2, ());
dma_trait_impl!(crate::timer::Ch4Dma, TIM3, DMA1_CH3, ());
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH3, ());
dma_trait_impl!(crate::timer::Ch1Dma, TIM3, DMA1_CH4, ());
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH3, ());
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH2, ());
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH3, ());
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH4, ());
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH5, ());
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH4, ());
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH5, ());
impl core::ops::Div<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 1u32 / 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 512u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllmul> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllmul) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllmul::MUL2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllmul::MUL3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllmul::MUL4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllmul::MUL5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllmul::MUL6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllmul::MUL7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllmul::MUL8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Pllmul::MUL9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Pllmul::MUL10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Pllmul::MUL11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Pllmul::MUL12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Pllmul::MUL13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Pllmul::MUL14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Pllmul::MUL15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Pllmul::MUL16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllmul> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllmul) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllmul::MUL2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllxtpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllxtpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllxtpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllxtpre::DIV2 => self * 1u32 / 2u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllxtpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllxtpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllxtpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllxtpre::DIV2 => self * 2u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Prediv> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Prediv) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Prediv::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Prediv::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Prediv::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Prediv::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Prediv::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Prediv::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Prediv::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Prediv::DIV9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Prediv::DIV10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Prediv::DIV11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Prediv::DIV12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Prediv::DIV13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Prediv::DIV14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Prediv::DIV15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Prediv::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Prediv> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Prediv) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Prediv::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Prediv::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod ADC1_COMMON {}
    pub mod CEC {
        pub type GLOBAL = crate::interrupt::typelevel::CEC_CAN;
    }
    pub mod COMP1 {
        pub type WKUP = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod COMP2 {
        pub type WKUP = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod CRC {}
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod DBGMCU {}
    pub mod DMA1 {
        pub type CH1 = crate::interrupt::typelevel::DMA1_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_CHANNEL2_3;
        pub type CH3 = crate::interrupt::typelevel::DMA1_CHANNEL2_3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_CHANNEL4_5;
        pub type CH5 = crate::interrupt::typelevel::DMA1_CHANNEL4_5;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0_1;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI0_1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2_3;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI2_3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI4_15;
    }
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOF {}
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1;
        pub type EV = crate::interrupt::typelevel::I2C1;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2;
        pub type EV = crate::interrupt::typelevel::I2C2;
    }
    pub mod IWDG {}
    pub mod PWR {}
    pub mod RCC {
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC;
        pub type SSRU = crate::interrupt::typelevel::RTC;
        pub type STAMP = crate::interrupt::typelevel::RTC;
        pub type TAMP = crate::interrupt::typelevel::RTC;
        pub type WKUP = crate::interrupt::typelevel::RTC;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod SYSCFG {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
        pub type TRG = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
        pub type UP = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
    }
    pub mod TIM14 {
        pub type BRK = crate::interrupt::typelevel::TIM14;
        pub type CC = crate::interrupt::typelevel::TIM14;
        pub type COM = crate::interrupt::typelevel::TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM14;
        pub type UP = crate::interrupt::typelevel::TIM14;
    }
    pub mod TIM15 {
        pub type BRK = crate::interrupt::typelevel::TIM15;
        pub type CC = crate::interrupt::typelevel::TIM15;
        pub type COM = crate::interrupt::typelevel::TIM15;
        pub type TRG = crate::interrupt::typelevel::TIM15;
        pub type UP = crate::interrupt::typelevel::TIM15;
    }
    pub mod TIM16 {
        pub type BRK = crate::interrupt::typelevel::TIM16;
        pub type CC = crate::interrupt::typelevel::TIM16;
        pub type COM = crate::interrupt::typelevel::TIM16;
        pub type TRG = crate::interrupt::typelevel::TIM16;
        pub type UP = crate::interrupt::typelevel::TIM16;
    }
    pub mod TIM17 {
        pub type BRK = crate::interrupt::typelevel::TIM17;
        pub type CC = crate::interrupt::typelevel::TIM17;
        pub type COM = crate::interrupt::typelevel::TIM17;
        pub type TRG = crate::interrupt::typelevel::TIM17;
        pub type UP = crate::interrupt::typelevel::TIM17;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod TSC {
        pub type GLOBAL = crate::interrupt::typelevel::TSC;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH1, 0u8);
dma_channel_impl!(DMA1_CH2, 1u8);
dma_channel_impl!(DMA1_CH3, 2u8);
dma_channel_impl!(DMA1_CH4, 3u8);
dma_channel_impl!(DMA1_CH5, 4u8);
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL1() {
    <crate::peripherals::DMA1_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL2_3() {
    <crate::peripherals::DMA1_CH2 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA1_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL4_5() {
    <crate::peripherals::DMA1_CH4 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA1_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 0usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 1usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 2usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 3usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 4usize,
    },
];
pub fn gpio_block(n: usize) -> crate::pac::gpio::Gpio {
    {
        unsafe {
            {
                crate::pac::gpio::Gpio::from_ptr((1207959552usize + 1024usize * n) as _)
            }
        }
    }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 65536usize;
pub const WRITE_SIZE: usize = 4usize;

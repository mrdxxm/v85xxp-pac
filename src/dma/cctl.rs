#[doc = "Register `C%sCTL` reader"]
pub struct R(crate::R<CCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C%sCTL` writer"]
pub struct W(crate::W<CCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - DMA channel enable register."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - DMA channel enable register."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - Transfer size mode."]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "Transfer size mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: `0`"]
    BYTE = 0,
    #[doc = "1: `1`"]
    HALF_WORD = 1,
    #[doc = "2: `10`"]
    WORD = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZE_A {
        match self.bits {
            0 => SIZE_A::BYTE,
            1 => SIZE_A::HALF_WORD,
            2 => SIZE_A::WORD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SIZE_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZE_A::WORD
    }
}
#[doc = "Field `SIZE` writer - Transfer size mode."]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCTL_SPEC, u8, SIZE_A, 2, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZE_A::BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SIZE_A::HALF_WORD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZE_A::WORD)
    }
}
#[doc = "Field `SMODE` reader - Source address mode."]
pub type SMODE_R = crate::FieldReader<u8, SMODE_A>;
#[doc = "Source address mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMODE_A {
    #[doc = "0: `0`"]
    FIX = 0,
    #[doc = "1: `1`"]
    INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END = 1,
    #[doc = "2: `10`"]
    INCREMENTAL_AND_ROUNDED_AT_FRAME_END = 2,
}
impl From<SMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMODE_A) -> Self {
        variant as _
    }
}
impl SMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMODE_A {
        match self.bits {
            0 => SMODE_A::FIX,
            1 => SMODE_A::INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END,
            2 => SMODE_A::INCREMENTAL_AND_ROUNDED_AT_FRAME_END,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIX`"]
    #[inline(always)]
    pub fn is_fix(&self) -> bool {
        *self == SMODE_A::FIX
    }
    #[doc = "Checks if the value of the field is `INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END`"]
    #[inline(always)]
    pub fn is_incremental_and_rounded_at_package_end(&self) -> bool {
        *self == SMODE_A::INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END
    }
    #[doc = "Checks if the value of the field is `INCREMENTAL_AND_ROUNDED_AT_FRAME_END`"]
    #[inline(always)]
    pub fn is_incremental_and_rounded_at_frame_end(&self) -> bool {
        *self == SMODE_A::INCREMENTAL_AND_ROUNDED_AT_FRAME_END
    }
}
#[doc = "Field `SMODE` writer - Source address mode."]
pub type SMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCTL_SPEC, u8, SMODE_A, 2, O>;
impl<'a, const O: u8> SMODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fix(self) -> &'a mut W {
        self.variant(SMODE_A::FIX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incremental_and_rounded_at_package_end(self) -> &'a mut W {
        self.variant(SMODE_A::INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incremental_and_rounded_at_frame_end(self) -> &'a mut W {
        self.variant(SMODE_A::INCREMENTAL_AND_ROUNDED_AT_FRAME_END)
    }
}
#[doc = "Field `DMODE` reader - Destination address mode."]
pub type DMODE_R = crate::FieldReader<u8, DMODE_A>;
#[doc = "Destination address mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMODE_A {
    #[doc = "0: `0`"]
    FIX = 0,
    #[doc = "1: `1`"]
    INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END = 1,
    #[doc = "2: `10`"]
    INCREMENTAL_AND_ROUNDED_AT_FRAME_END = 2,
}
impl From<DMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMODE_A) -> Self {
        variant as _
    }
}
impl DMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMODE_A {
        match self.bits {
            0 => DMODE_A::FIX,
            1 => DMODE_A::INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END,
            2 => DMODE_A::INCREMENTAL_AND_ROUNDED_AT_FRAME_END,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIX`"]
    #[inline(always)]
    pub fn is_fix(&self) -> bool {
        *self == DMODE_A::FIX
    }
    #[doc = "Checks if the value of the field is `INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END`"]
    #[inline(always)]
    pub fn is_incremental_and_rounded_at_package_end(&self) -> bool {
        *self == DMODE_A::INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END
    }
    #[doc = "Checks if the value of the field is `INCREMENTAL_AND_ROUNDED_AT_FRAME_END`"]
    #[inline(always)]
    pub fn is_incremental_and_rounded_at_frame_end(&self) -> bool {
        *self == DMODE_A::INCREMENTAL_AND_ROUNDED_AT_FRAME_END
    }
}
#[doc = "Field `DMODE` writer - Destination address mode."]
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCTL_SPEC, u8, DMODE_A, 2, O>;
impl<'a, const O: u8> DMODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fix(self) -> &'a mut W {
        self.variant(DMODE_A::FIX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incremental_and_rounded_at_package_end(self) -> &'a mut W {
        self.variant(DMODE_A::INCREMENTAL_AND_ROUNDED_AT_PACKAGE_END)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incremental_and_rounded_at_frame_end(self) -> &'a mut W {
        self.variant(DMODE_A::INCREMENTAL_AND_ROUNDED_AT_FRAME_END)
    }
}
#[doc = "Field `DMASEL` reader - DMA request source selection."]
pub type DMASEL_R = crate::FieldReader<u8, DMASEL_A>;
#[doc = "DMA request source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASEL_A {
    #[doc = "0: `0`"]
    SOFTWARE = 0,
    #[doc = "1: `1`"]
    ADC = 1,
    #[doc = "2: `10`"]
    UART0_TX = 2,
    #[doc = "3: `11`"]
    UART0_RX = 3,
    #[doc = "4: `100`"]
    UART1_TX = 4,
    #[doc = "5: `101`"]
    UART1_RX = 5,
    #[doc = "6: `110`"]
    UART2_TX = 6,
    #[doc = "7: `111`"]
    UART2_RX = 7,
    #[doc = "8: `1000`"]
    UART3_TX = 8,
    #[doc = "9: `1001`"]
    UART3_RX = 9,
    #[doc = "10: `1010`"]
    UART4_TX = 10,
    #[doc = "11: `1011`"]
    UART4_RX = 11,
    #[doc = "12: `1100`"]
    UART5_TX = 12,
    #[doc = "13: `1101`"]
    UART5_RX = 13,
    #[doc = "14: `1110`"]
    ISO78160_TX = 14,
    #[doc = "15: `1111`"]
    ISO78160_RX = 15,
    #[doc = "16: `10000`"]
    ISO78161_TX = 16,
    #[doc = "17: `10001`"]
    ISO78161_RX = 17,
    #[doc = "18: `10010`"]
    TIMER0 = 18,
    #[doc = "19: `10011`"]
    TIMER1 = 19,
    #[doc = "20: `10100`"]
    TIMER2 = 20,
    #[doc = "21: `10101`"]
    TIMER3 = 21,
    #[doc = "22: `10110`"]
    SPI1_TX = 22,
    #[doc = "23: `10111`"]
    SPI1_RX = 23,
    #[doc = "24: `11000`"]
    UART_32K0 = 24,
    #[doc = "25: `11001`"]
    UART_32K1 = 25,
    #[doc = "26: `11010`"]
    CMP1 = 26,
    #[doc = "27: `11011`"]
    CMP2 = 27,
    #[doc = "28: `11100`"]
    SPI3_TX = 28,
    #[doc = "29: `11101`"]
    SPI3_RX = 29,
    #[doc = "30: `11110`"]
    SPI2_TX = 30,
    #[doc = "31: `11111`"]
    SPI2_RX = 31,
}
impl From<DMASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASEL_A) -> Self {
        variant as _
    }
}
impl DMASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASEL_A {
        match self.bits {
            0 => DMASEL_A::SOFTWARE,
            1 => DMASEL_A::ADC,
            2 => DMASEL_A::UART0_TX,
            3 => DMASEL_A::UART0_RX,
            4 => DMASEL_A::UART1_TX,
            5 => DMASEL_A::UART1_RX,
            6 => DMASEL_A::UART2_TX,
            7 => DMASEL_A::UART2_RX,
            8 => DMASEL_A::UART3_TX,
            9 => DMASEL_A::UART3_RX,
            10 => DMASEL_A::UART4_TX,
            11 => DMASEL_A::UART4_RX,
            12 => DMASEL_A::UART5_TX,
            13 => DMASEL_A::UART5_RX,
            14 => DMASEL_A::ISO78160_TX,
            15 => DMASEL_A::ISO78160_RX,
            16 => DMASEL_A::ISO78161_TX,
            17 => DMASEL_A::ISO78161_RX,
            18 => DMASEL_A::TIMER0,
            19 => DMASEL_A::TIMER1,
            20 => DMASEL_A::TIMER2,
            21 => DMASEL_A::TIMER3,
            22 => DMASEL_A::SPI1_TX,
            23 => DMASEL_A::SPI1_RX,
            24 => DMASEL_A::UART_32K0,
            25 => DMASEL_A::UART_32K1,
            26 => DMASEL_A::CMP1,
            27 => DMASEL_A::CMP2,
            28 => DMASEL_A::SPI3_TX,
            29 => DMASEL_A::SPI3_RX,
            30 => DMASEL_A::SPI2_TX,
            31 => DMASEL_A::SPI2_RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == DMASEL_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DMASEL_A::ADC
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == DMASEL_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == DMASEL_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == DMASEL_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == DMASEL_A::UART1_RX
    }
    #[doc = "Checks if the value of the field is `UART2_TX`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == DMASEL_A::UART2_TX
    }
    #[doc = "Checks if the value of the field is `UART2_RX`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == DMASEL_A::UART2_RX
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == DMASEL_A::UART3_TX
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == DMASEL_A::UART3_RX
    }
    #[doc = "Checks if the value of the field is `UART4_TX`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == DMASEL_A::UART4_TX
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == DMASEL_A::UART4_RX
    }
    #[doc = "Checks if the value of the field is `UART5_TX`"]
    #[inline(always)]
    pub fn is_uart5_tx(&self) -> bool {
        *self == DMASEL_A::UART5_TX
    }
    #[doc = "Checks if the value of the field is `UART5_RX`"]
    #[inline(always)]
    pub fn is_uart5_rx(&self) -> bool {
        *self == DMASEL_A::UART5_RX
    }
    #[doc = "Checks if the value of the field is `ISO78160_TX`"]
    #[inline(always)]
    pub fn is_iso78160_tx(&self) -> bool {
        *self == DMASEL_A::ISO78160_TX
    }
    #[doc = "Checks if the value of the field is `ISO78160_RX`"]
    #[inline(always)]
    pub fn is_iso78160_rx(&self) -> bool {
        *self == DMASEL_A::ISO78160_RX
    }
    #[doc = "Checks if the value of the field is `ISO78161_TX`"]
    #[inline(always)]
    pub fn is_iso78161_tx(&self) -> bool {
        *self == DMASEL_A::ISO78161_TX
    }
    #[doc = "Checks if the value of the field is `ISO78161_RX`"]
    #[inline(always)]
    pub fn is_iso78161_rx(&self) -> bool {
        *self == DMASEL_A::ISO78161_RX
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == DMASEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == DMASEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == DMASEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == DMASEL_A::TIMER3
    }
    #[doc = "Checks if the value of the field is `SPI1_TX`"]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == DMASEL_A::SPI1_TX
    }
    #[doc = "Checks if the value of the field is `SPI1_RX`"]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == DMASEL_A::SPI1_RX
    }
    #[doc = "Checks if the value of the field is `UART_32K0`"]
    #[inline(always)]
    pub fn is_uart_32k0(&self) -> bool {
        *self == DMASEL_A::UART_32K0
    }
    #[doc = "Checks if the value of the field is `UART_32K1`"]
    #[inline(always)]
    pub fn is_uart_32k1(&self) -> bool {
        *self == DMASEL_A::UART_32K1
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        *self == DMASEL_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP2`"]
    #[inline(always)]
    pub fn is_cmp2(&self) -> bool {
        *self == DMASEL_A::CMP2
    }
    #[doc = "Checks if the value of the field is `SPI3_TX`"]
    #[inline(always)]
    pub fn is_spi3_tx(&self) -> bool {
        *self == DMASEL_A::SPI3_TX
    }
    #[doc = "Checks if the value of the field is `SPI3_RX`"]
    #[inline(always)]
    pub fn is_spi3_rx(&self) -> bool {
        *self == DMASEL_A::SPI3_RX
    }
    #[doc = "Checks if the value of the field is `SPI2_TX`"]
    #[inline(always)]
    pub fn is_spi2_tx(&self) -> bool {
        *self == DMASEL_A::SPI2_TX
    }
    #[doc = "Checks if the value of the field is `SPI2_RX`"]
    #[inline(always)]
    pub fn is_spi2_rx(&self) -> bool {
        *self == DMASEL_A::SPI2_RX
    }
}
#[doc = "Field `DMASEL` writer - DMA request source selection."]
pub type DMASEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCTL_SPEC, u8, DMASEL_A, 5, O>;
impl<'a, const O: u8> DMASEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(DMASEL_A::SOFTWARE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(DMASEL_A::ADC)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART0_TX)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART0_RX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART1_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART1_RX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART2_TX)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART2_RX)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART3_TX)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART3_RX)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART4_TX)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART4_RX)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn uart5_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART5_TX)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn uart5_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::UART5_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn iso78160_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::ISO78160_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn iso78160_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::ISO78160_RX)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn iso78161_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::ISO78161_TX)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn iso78161_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::ISO78161_RX)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(DMASEL_A::TIMER0)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(DMASEL_A::TIMER1)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(DMASEL_A::TIMER2)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut W {
        self.variant(DMASEL_A::TIMER3)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::SPI1_TX)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::SPI1_RX)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn uart_32k0(self) -> &'a mut W {
        self.variant(DMASEL_A::UART_32K0)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn uart_32k1(self) -> &'a mut W {
        self.variant(DMASEL_A::UART_32K1)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(DMASEL_A::CMP1)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn cmp2(self) -> &'a mut W {
        self.variant(DMASEL_A::CMP2)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn spi3_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::SPI3_TX)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn spi3_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::SPI3_RX)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn spi2_tx(self) -> &'a mut W {
        self.variant(DMASEL_A::SPI2_TX)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn spi2_rx(self) -> &'a mut W {
        self.variant(DMASEL_A::SPI2_RX)
    }
}
#[doc = "Field `TMODE` reader - Transmit mode."]
pub type TMODE_R = crate::BitReader<bool>;
#[doc = "Field `TMODE` writer - Transmit mode."]
pub type TMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `CONT` reader - Continuous mode."]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - Continuous mode."]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `AESEN` reader - Enable AES encrypt/decrypt function of DMA channel. Only DMA channel 3 supports AES function."]
pub type AESEN_R = crate::BitReader<bool>;
#[doc = "Field `AESEN` writer - Enable AES encrypt/decrypt function of DMA channel. Only DMA channel 3 supports AES function."]
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Force stop DMA transfer."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Force stop DMA transfer."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `PLEN` reader - Package length register actual transfer package length is (PLEN + 1)."]
pub type PLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLEN` writer - Package length register actual transfer package length is (PLEN + 1)."]
pub type PLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `FLEN` reader - Frame length register actual transfer frame length is (FLEN + 1)."]
pub type FLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLEN` writer - Frame length register actual transfer frame length is (FLEN + 1)."]
pub type FLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - DMA channel enable register."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Transfer size mode."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Source address mode."]
    #[inline(always)]
    pub fn smode(&self) -> SMODE_R {
        SMODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Destination address mode."]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:11 - DMA request source selection."]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - Transmit mode."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Continuous mode."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable AES encrypt/decrypt function of DMA channel. Only DMA channel 3 supports AES function."]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Force stop DMA transfer."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Package length register actual transfer package length is (PLEN + 1)."]
    #[inline(always)]
    pub fn plen(&self) -> PLEN_R {
        PLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame length register actual transfer frame length is (FLEN + 1)."]
    #[inline(always)]
    pub fn flen(&self) -> FLEN_R {
        FLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA channel enable register."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Transfer size mode."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<1> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 3:4 - Source address mode."]
    #[inline(always)]
    #[must_use]
    pub fn smode(&mut self) -> SMODE_W<3> {
        SMODE_W::new(self)
    }
    #[doc = "Bits 5:6 - Destination address mode."]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<5> {
        DMODE_W::new(self)
    }
    #[doc = "Bits 7:11 - DMA request source selection."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<7> {
        DMASEL_W::new(self)
    }
    #[doc = "Bit 12 - Transmit mode."]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TMODE_W<12> {
        TMODE_W::new(self)
    }
    #[doc = "Bit 13 - Continuous mode."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    #[doc = "Bit 14 - Enable AES encrypt/decrypt function of DMA channel. Only DMA channel 3 supports AES function."]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<14> {
        AESEN_W::new(self)
    }
    #[doc = "Bit 15 - Force stop DMA transfer."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<15> {
        STOP_W::new(self)
    }
    #[doc = "Bits 16:23 - Package length register actual transfer package length is (PLEN + 1)."]
    #[inline(always)]
    #[must_use]
    pub fn plen(&mut self) -> PLEN_W<16> {
        PLEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame length register actual transfer frame length is (FLEN + 1)."]
    #[inline(always)]
    #[must_use]
    pub fn flen(&mut self) -> FLEN_W<24> {
        FLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel x control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctl](index.html) module"]
pub struct CCTL_SPEC;
impl crate::RegisterSpec for CCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cctl::R](R) reader structure"]
impl crate::Readable for CCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cctl::W](W) writer structure"]
impl crate::Writable for CCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C%sCTL to value 0"]
impl crate::Resettable for CCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

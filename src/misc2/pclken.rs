#[doc = "Register `PCLKEN` reader"]
pub struct R(crate::R<PCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLKEN` writer"]
pub struct W(crate::W<PCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLKEN_SPEC>;
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
impl From<crate::W<PCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - DMA Controller."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA Controller."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `I2C` reader - I2C Controller."]
pub type I2C_R = crate::BitReader<bool>;
#[doc = "Field `I2C` writer - I2C Controller."]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `SPI1` reader - SPI1 Controller."]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1` writer - SPI1 Controller."]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `UART0` reader - UART0 Controller."]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - UART0 Controller."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `UART1` reader - UART1 Controller."]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - UART1 Controller."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `UART2` reader - UART2 Controller."]
pub type UART2_R = crate::BitReader<bool>;
#[doc = "Field `UART2` writer - UART2 Controller."]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `UART3` reader - UART3 Controller."]
pub type UART3_R = crate::BitReader<bool>;
#[doc = "Field `UART3` writer - UART3 Controller."]
pub type UART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `UART4` reader - UART4 Controller."]
pub type UART4_R = crate::BitReader<bool>;
#[doc = "Field `UART4` writer - UART4 Controller."]
pub type UART4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `UART5` reader - UART5 Controller."]
pub type UART5_R = crate::BitReader<bool>;
#[doc = "Field `UART5` writer - UART5 Controller."]
pub type UART5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `ISO78160` reader - ISO78160 Controller."]
pub type ISO78160_R = crate::BitReader<bool>;
#[doc = "Field `ISO78160` writer - ISO78160 Controller."]
pub type ISO78160_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `ISO78161` reader - ISO78161 Controller."]
pub type ISO78161_R = crate::BitReader<bool>;
#[doc = "Field `ISO78161` writer - ISO78161 Controller."]
pub type ISO78161_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `TIMER` reader - Timer Controller."]
pub type TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TIMER` writer - Timer Controller."]
pub type TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `MISC1` reader - MISC1 Controller."]
pub type MISC1_R = crate::BitReader<bool>;
#[doc = "Field `MISC1` writer - MISC1 Controller."]
pub type MISC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `MISC2` reader - MISC2 Controller."]
pub type MISC2_R = crate::BitReader<bool>;
#[doc = "Field `MISC2` writer - MISC2 Controller."]
pub type MISC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `PMU` reader - PMU Controller."]
pub type PMU_R = crate::BitReader<bool>;
#[doc = "Field `PMU` writer - PMU Controller."]
pub type PMU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `RTC` reader - RTC Controller."]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - RTC Controller."]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `ANA` reader - ANA Controller."]
pub type ANA_R = crate::BitReader<bool>;
#[doc = "Field `ANA` writer - ANA Controller."]
pub type ANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `U32K0` reader - U32K0 Controller."]
pub type U32K0_R = crate::BitReader<bool>;
#[doc = "Field `U32K0` writer - U32K0 Controller."]
pub type U32K0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `U32K1` reader - U32K1 Controller."]
pub type U32K1_R = crate::BitReader<bool>;
#[doc = "Field `U32K1` writer - U32K1 Controller."]
pub type U32K1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `SPI2` reader - SPI2 Controller."]
pub type SPI2_R = crate::BitReader<bool>;
#[doc = "Field `SPI2` writer - SPI2 Controller."]
pub type SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
#[doc = "Field `SPI3` reader - SPI3 Controller."]
pub type SPI3_R = crate::BitReader<bool>;
#[doc = "Field `SPI3` writer - SPI3 Controller."]
pub type SPI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - DMA Controller."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Controller."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI1 Controller."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 Controller."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART1 Controller."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART2 Controller."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART3 Controller."]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART4 Controller."]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART5 Controller."]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ISO78160 Controller."]
    #[inline(always)]
    pub fn iso78160(&self) -> ISO78160_R {
        ISO78160_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ISO78161 Controller."]
    #[inline(always)]
    pub fn iso78161(&self) -> ISO78161_R {
        ISO78161_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer Controller."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MISC1 Controller."]
    #[inline(always)]
    pub fn misc1(&self) -> MISC1_R {
        MISC1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MISC2 Controller."]
    #[inline(always)]
    pub fn misc2(&self) -> MISC2_R {
        MISC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PMU Controller."]
    #[inline(always)]
    pub fn pmu(&self) -> PMU_R {
        PMU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC Controller."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ANA Controller."]
    #[inline(always)]
    pub fn ana(&self) -> ANA_R {
        ANA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - U32K0 Controller."]
    #[inline(always)]
    pub fn u32k0(&self) -> U32K0_R {
        U32K0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - U32K1 Controller."]
    #[inline(always)]
    pub fn u32k1(&self) -> U32K1_R {
        U32K1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI2 Controller."]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SPI3 Controller."]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA Controller."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<1> {
        DMA_W::new(self)
    }
    #[doc = "Bit 2 - I2C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<2> {
        I2C_W::new(self)
    }
    #[doc = "Bit 3 - SPI1 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<3> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 4 - UART0 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<4> {
        UART0_W::new(self)
    }
    #[doc = "Bit 5 - UART1 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<5> {
        UART1_W::new(self)
    }
    #[doc = "Bit 6 - UART2 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<6> {
        UART2_W::new(self)
    }
    #[doc = "Bit 7 - UART3 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<7> {
        UART3_W::new(self)
    }
    #[doc = "Bit 8 - UART4 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<8> {
        UART4_W::new(self)
    }
    #[doc = "Bit 9 - UART5 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<9> {
        UART5_W::new(self)
    }
    #[doc = "Bit 10 - ISO78160 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn iso78160(&mut self) -> ISO78160_W<10> {
        ISO78160_W::new(self)
    }
    #[doc = "Bit 11 - ISO78161 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn iso78161(&mut self) -> ISO78161_W<11> {
        ISO78161_W::new(self)
    }
    #[doc = "Bit 12 - Timer Controller."]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<12> {
        TIMER_W::new(self)
    }
    #[doc = "Bit 13 - MISC1 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn misc1(&mut self) -> MISC1_W<13> {
        MISC1_W::new(self)
    }
    #[doc = "Bit 14 - MISC2 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn misc2(&mut self) -> MISC2_W<14> {
        MISC2_W::new(self)
    }
    #[doc = "Bit 15 - PMU Controller."]
    #[inline(always)]
    #[must_use]
    pub fn pmu(&mut self) -> PMU_W<15> {
        PMU_W::new(self)
    }
    #[doc = "Bit 16 - RTC Controller."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<16> {
        RTC_W::new(self)
    }
    #[doc = "Bit 17 - ANA Controller."]
    #[inline(always)]
    #[must_use]
    pub fn ana(&mut self) -> ANA_W<17> {
        ANA_W::new(self)
    }
    #[doc = "Bit 18 - U32K0 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn u32k0(&mut self) -> U32K0_W<18> {
        U32K0_W::new(self)
    }
    #[doc = "Bit 19 - U32K1 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn u32k1(&mut self) -> U32K1_W<19> {
        U32K1_W::new(self)
    }
    #[doc = "Bit 21 - SPI2 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<21> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 22 - SPI3 Controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<22> {
        SPI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB clock enable control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclken](index.html) module"]
pub struct PCLKEN_SPEC;
impl crate::RegisterSpec for PCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclken::R](R) reader structure"]
impl crate::Readable for PCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclken::W](W) writer structure"]
impl crate::Writable for PCLKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKEN to value 0xffff_ffff"]
impl crate::Resettable for PCLKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

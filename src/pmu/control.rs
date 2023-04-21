#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_IOA_EN` reader - IOA0~15 interrupt enable register."]
pub type INT_IOA_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_IOA_EN` writer - IOA0~15 interrupt enable register."]
pub type INT_IOA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `RTCCLK_SEL` reader - RTC Clock selection."]
pub type RTCCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTCCLK_SEL` writer - RTC Clock selection."]
pub type RTCCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `INT_32K_EN` reader - 32K XTAL absent interrupt enable register."]
pub type INT_32K_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_32K_EN` writer - 32K XTAL absent interrupt enable register."]
pub type INT_32K_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `INT_6M_EN` reader - 6.5536M XTAL absent interrupt enable register."]
pub type INT_6M_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_6M_EN` writer - 6.5536M XTAL absent interrupt enable register."]
pub type INT_6M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `PLLH_SEL` reader - High speed PLL input clock selection."]
pub type PLLH_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLH_SEL` writer - High speed PLL input clock selection."]
pub type PLLH_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `PLLL_SEL` reader - Low speed PLL input clock selection."]
pub type PLLL_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLL_SEL` writer - Low speed PLL input clock selection."]
pub type PLLL_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `PWUPCYC` reader - Power-up cycle count."]
pub type PWUPCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWUPCYC` writer - Power-up cycle count."]
pub type PWUPCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 8, O>;
#[doc = "Field `FORCE_CLKSEL_RCH` reader - Wake up clock selection for idle mode."]
pub type FORCE_CLKSEL_RCH_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_CLKSEL_RCH` writer - Wake up clock selection for idle mode."]
pub type FORCE_CLKSEL_RCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IOA0~15 interrupt enable register."]
    #[inline(always)]
    pub fn int_ioa_en(&self) -> INT_IOA_EN_R {
        INT_IOA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Clock selection."]
    #[inline(always)]
    pub fn rtcclk_sel(&self) -> RTCCLK_SEL_R {
        RTCCLK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32K XTAL absent interrupt enable register."]
    #[inline(always)]
    pub fn int_32k_en(&self) -> INT_32K_EN_R {
        INT_32K_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 6.5536M XTAL absent interrupt enable register."]
    #[inline(always)]
    pub fn int_6m_en(&self) -> INT_6M_EN_R {
        INT_6M_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High speed PLL input clock selection."]
    #[inline(always)]
    pub fn pllh_sel(&self) -> PLLH_SEL_R {
        PLLH_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low speed PLL input clock selection."]
    #[inline(always)]
    pub fn plll_sel(&self) -> PLLL_SEL_R {
        PLLL_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Power-up cycle count."]
    #[inline(always)]
    pub fn pwupcyc(&self) -> PWUPCYC_R {
        PWUPCYC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 20 - Wake up clock selection for idle mode."]
    #[inline(always)]
    pub fn force_clksel_rch(&self) -> FORCE_CLKSEL_RCH_R {
        FORCE_CLKSEL_RCH_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOA0~15 interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn int_ioa_en(&mut self) -> INT_IOA_EN_W<0> {
        INT_IOA_EN_W::new(self)
    }
    #[doc = "Bit 1 - RTC Clock selection."]
    #[inline(always)]
    #[must_use]
    pub fn rtcclk_sel(&mut self) -> RTCCLK_SEL_W<1> {
        RTCCLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - 32K XTAL absent interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn int_32k_en(&mut self) -> INT_32K_EN_W<2> {
        INT_32K_EN_W::new(self)
    }
    #[doc = "Bit 3 - 6.5536M XTAL absent interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn int_6m_en(&mut self) -> INT_6M_EN_W<3> {
        INT_6M_EN_W::new(self)
    }
    #[doc = "Bit 4 - High speed PLL input clock selection."]
    #[inline(always)]
    #[must_use]
    pub fn pllh_sel(&mut self) -> PLLH_SEL_W<4> {
        PLLH_SEL_W::new(self)
    }
    #[doc = "Bit 5 - Low speed PLL input clock selection."]
    #[inline(always)]
    #[must_use]
    pub fn plll_sel(&mut self) -> PLLL_SEL_W<5> {
        PLLL_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Power-up cycle count."]
    #[inline(always)]
    #[must_use]
    pub fn pwupcyc(&mut self) -> PWUPCYC_W<8> {
        PWUPCYC_W::new(self)
    }
    #[doc = "Bit 20 - Wake up clock selection for idle mode."]
    #[inline(always)]
    #[must_use]
    pub fn force_clksel_rch(&mut self) -> FORCE_CLKSEL_RCH_W<20> {
        FORCE_CLKSEL_RCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

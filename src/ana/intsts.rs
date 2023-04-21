#[doc = "Register `INTSTS` reader"]
pub struct R(crate::R<INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS` writer"]
pub struct W(crate::W<INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS_SPEC>;
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
impl From<crate::W<INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTSTS0` reader - Interrupt flag of manual ADC conversion done."]
pub type INTSTS0_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS0` writer - Interrupt flag of manual ADC conversion done."]
pub type INTSTS0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS1` reader - Interrupt flag of auto ADC conversion done."]
pub type INTSTS1_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS1` writer - Interrupt flag of auto ADC conversion done."]
pub type INTSTS1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS2` reader - Interrupt flag of CMP1."]
pub type INTSTS2_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS2` writer - Interrupt flag of CMP1."]
pub type INTSTS2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS3` reader - Interrupt flag of CMP2."]
pub type INTSTS3_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS3` writer - Interrupt flag of CMP2."]
pub type INTSTS3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS7` reader - Interrupt flag of POWALARM."]
pub type INTSTS7_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS7` writer - Interrupt flag of POWALARM."]
pub type INTSTS7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS8` reader - Interrupt flag of PWRDROP."]
pub type INTSTS8_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS8` writer - Interrupt flag of PWRDROP."]
pub type INTSTS8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS10` reader - Interrupt flag of POWLV, this interrupt will be generated when POWLV rising or falling."]
pub type INTSTS10_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS10` writer - Interrupt flag of POWLV, this interrupt will be generated when POWLV rising or falling."]
pub type INTSTS10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS11` reader - Interrupt flag of sleep mode entry under PWRDROP is 0."]
pub type INTSTS11_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS11` writer - Interrupt flag of sleep mode entry under PWRDROP is 0."]
pub type INTSTS11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS12` reader - ANA_REGx error flag."]
pub type INTSTS12_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS12` writer - ANA_REGx error flag."]
pub type INTSTS12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS13` reader - TADC change over-threshold interrupt."]
pub type INTSTS13_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS13` writer - TADC change over-threshold interrupt."]
pub type INTSTS13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS14` reader - ADC data below LOWER_THD0."]
pub type INTSTS14_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS14` writer - ADC data below LOWER_THD0."]
pub type INTSTS14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS15` reader - ADC data over UPPER_THD0."]
pub type INTSTS15_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS15` writer - ADC data over UPPER_THD0."]
pub type INTSTS15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS16` reader - ADC data below LOWER_THD1."]
pub type INTSTS16_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS16` writer - ADC data below LOWER_THD1."]
pub type INTSTS16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS17` reader - ADC data over UPPER_THD1."]
pub type INTSTS17_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS17` writer - ADC data over UPPER_THD1."]
pub type INTSTS17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS18` reader - ADC data below LOWER_THD2."]
pub type INTSTS18_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS18` writer - ADC data below LOWER_THD2."]
pub type INTSTS18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS19` reader - ADC data over UPPER_THD2."]
pub type INTSTS19_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS19` writer - ADC data over UPPER_THD2."]
pub type INTSTS19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS20` reader - ADC data over LOWER_THD3."]
pub type INTSTS20_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS20` writer - ADC data over LOWER_THD3."]
pub type INTSTS20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS21` reader - ADC data over UPPER_THD3."]
pub type INTSTS21_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS21` writer - ADC data over UPPER_THD3."]
pub type INTSTS21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt flag of manual ADC conversion done."]
    #[inline(always)]
    pub fn intsts0(&self) -> INTSTS0_R {
        INTSTS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag of auto ADC conversion done."]
    #[inline(always)]
    pub fn intsts1(&self) -> INTSTS1_R {
        INTSTS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag of CMP1."]
    #[inline(always)]
    pub fn intsts2(&self) -> INTSTS2_R {
        INTSTS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag of CMP2."]
    #[inline(always)]
    pub fn intsts3(&self) -> INTSTS3_R {
        INTSTS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt flag of POWALARM."]
    #[inline(always)]
    pub fn intsts7(&self) -> INTSTS7_R {
        INTSTS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt flag of PWRDROP."]
    #[inline(always)]
    pub fn intsts8(&self) -> INTSTS8_R {
        INTSTS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag of POWLV, this interrupt will be generated when POWLV rising or falling."]
    #[inline(always)]
    pub fn intsts10(&self) -> INTSTS10_R {
        INTSTS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt flag of sleep mode entry under PWRDROP is 0."]
    #[inline(always)]
    pub fn intsts11(&self) -> INTSTS11_R {
        INTSTS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ANA_REGx error flag."]
    #[inline(always)]
    pub fn intsts12(&self) -> INTSTS12_R {
        INTSTS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TADC change over-threshold interrupt."]
    #[inline(always)]
    pub fn intsts13(&self) -> INTSTS13_R {
        INTSTS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC data below LOWER_THD0."]
    #[inline(always)]
    pub fn intsts14(&self) -> INTSTS14_R {
        INTSTS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC data over UPPER_THD0."]
    #[inline(always)]
    pub fn intsts15(&self) -> INTSTS15_R {
        INTSTS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC data below LOWER_THD1."]
    #[inline(always)]
    pub fn intsts16(&self) -> INTSTS16_R {
        INTSTS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC data over UPPER_THD1."]
    #[inline(always)]
    pub fn intsts17(&self) -> INTSTS17_R {
        INTSTS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC data below LOWER_THD2."]
    #[inline(always)]
    pub fn intsts18(&self) -> INTSTS18_R {
        INTSTS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC data over UPPER_THD2."]
    #[inline(always)]
    pub fn intsts19(&self) -> INTSTS19_R {
        INTSTS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC data over LOWER_THD3."]
    #[inline(always)]
    pub fn intsts20(&self) -> INTSTS20_R {
        INTSTS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC data over UPPER_THD3."]
    #[inline(always)]
    pub fn intsts21(&self) -> INTSTS21_R {
        INTSTS21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag of manual ADC conversion done."]
    #[inline(always)]
    #[must_use]
    pub fn intsts0(&mut self) -> INTSTS0_W<0> {
        INTSTS0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt flag of auto ADC conversion done."]
    #[inline(always)]
    #[must_use]
    pub fn intsts1(&mut self) -> INTSTS1_W<1> {
        INTSTS1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt flag of CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn intsts2(&mut self) -> INTSTS2_W<2> {
        INTSTS2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt flag of CMP2."]
    #[inline(always)]
    #[must_use]
    pub fn intsts3(&mut self) -> INTSTS3_W<3> {
        INTSTS3_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt flag of POWALARM."]
    #[inline(always)]
    #[must_use]
    pub fn intsts7(&mut self) -> INTSTS7_W<7> {
        INTSTS7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt flag of PWRDROP."]
    #[inline(always)]
    #[must_use]
    pub fn intsts8(&mut self) -> INTSTS8_W<8> {
        INTSTS8_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt flag of POWLV, this interrupt will be generated when POWLV rising or falling."]
    #[inline(always)]
    #[must_use]
    pub fn intsts10(&mut self) -> INTSTS10_W<10> {
        INTSTS10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt flag of sleep mode entry under PWRDROP is 0."]
    #[inline(always)]
    #[must_use]
    pub fn intsts11(&mut self) -> INTSTS11_W<11> {
        INTSTS11_W::new(self)
    }
    #[doc = "Bit 12 - ANA_REGx error flag."]
    #[inline(always)]
    #[must_use]
    pub fn intsts12(&mut self) -> INTSTS12_W<12> {
        INTSTS12_W::new(self)
    }
    #[doc = "Bit 13 - TADC change over-threshold interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intsts13(&mut self) -> INTSTS13_W<13> {
        INTSTS13_W::new(self)
    }
    #[doc = "Bit 14 - ADC data below LOWER_THD0."]
    #[inline(always)]
    #[must_use]
    pub fn intsts14(&mut self) -> INTSTS14_W<14> {
        INTSTS14_W::new(self)
    }
    #[doc = "Bit 15 - ADC data over UPPER_THD0."]
    #[inline(always)]
    #[must_use]
    pub fn intsts15(&mut self) -> INTSTS15_W<15> {
        INTSTS15_W::new(self)
    }
    #[doc = "Bit 16 - ADC data below LOWER_THD1."]
    #[inline(always)]
    #[must_use]
    pub fn intsts16(&mut self) -> INTSTS16_W<16> {
        INTSTS16_W::new(self)
    }
    #[doc = "Bit 17 - ADC data over UPPER_THD1."]
    #[inline(always)]
    #[must_use]
    pub fn intsts17(&mut self) -> INTSTS17_W<17> {
        INTSTS17_W::new(self)
    }
    #[doc = "Bit 18 - ADC data below LOWER_THD2."]
    #[inline(always)]
    #[must_use]
    pub fn intsts18(&mut self) -> INTSTS18_W<18> {
        INTSTS18_W::new(self)
    }
    #[doc = "Bit 19 - ADC data over UPPER_THD2."]
    #[inline(always)]
    #[must_use]
    pub fn intsts19(&mut self) -> INTSTS19_W<19> {
        INTSTS19_W::new(self)
    }
    #[doc = "Bit 20 - ADC data over LOWER_THD3."]
    #[inline(always)]
    #[must_use]
    pub fn intsts20(&mut self) -> INTSTS20_W<20> {
        INTSTS20_W::new(self)
    }
    #[doc = "Bit 21 - ADC data over UPPER_THD3."]
    #[inline(always)]
    #[must_use]
    pub fn intsts21(&mut self) -> INTSTS21_W<21> {
        INTSTS21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts](index.html) module"]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsts::R](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts::W](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x003f_fd8f;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

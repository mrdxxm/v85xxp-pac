#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN0` reader - Interrupt enable control of manual ADC conversion done."]
pub type INTEN0_R = crate::BitReader<bool>;
#[doc = "Field `INTEN0` writer - Interrupt enable control of manual ADC conversion done."]
pub type INTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN1` reader - Interrupt enable control of auto ADC conversion done."]
pub type INTEN1_R = crate::BitReader<bool>;
#[doc = "Field `INTEN1` writer - Interrupt enable control of auto ADC conversion done."]
pub type INTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN2` reader - Interrupt and wake-up enable control of CMP1."]
pub type INTEN2_R = crate::BitReader<bool>;
#[doc = "Field `INTEN2` writer - Interrupt and wake-up enable control of CMP1."]
pub type INTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN3` reader - Interrupt and wake-up enable control of CMP2."]
pub type INTEN3_R = crate::BitReader<bool>;
#[doc = "Field `INTEN3` writer - Interrupt and wake-up enable control of CMP2."]
pub type INTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN7` reader - Interrupt and wake-up enable control of POWALARM."]
pub type INTEN7_R = crate::BitReader<bool>;
#[doc = "Field `INTEN7` writer - Interrupt and wake-up enable control of POWALARM."]
pub type INTEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN8` reader - Interrupt and wake-up enable control of PWRDROP."]
pub type INTEN8_R = crate::BitReader<bool>;
#[doc = "Field `INTEN8` writer - Interrupt and wake-up enable control of PWRDROP."]
pub type INTEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN10` reader - Interrupt and wake-up enable control of POWLV."]
pub type INTEN10_R = crate::BitReader<bool>;
#[doc = "Field `INTEN10` writer - Interrupt and wake-up enable control of POWLV."]
pub type INTEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN11` reader - Interrupt and wake-up enable control of sleep mode entry, when VDCINDROP is 0."]
pub type INTEN11_R = crate::BitReader<bool>;
#[doc = "Field `INTEN11` writer - Interrupt and wake-up enable control of sleep mode entry, when VDCINDROP is 0."]
pub type INTEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN12` reader - Interrupt and wake-up enable control of ANA_REGx error."]
pub type INTEN12_R = crate::BitReader<bool>;
#[doc = "Field `INTEN12` writer - Interrupt and wake-up enable control of ANA_REGx error."]
pub type INTEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN13` reader - Interrupt and wake-up enable control of TADC change over threshold."]
pub type INTEN13_R = crate::BitReader<bool>;
#[doc = "Field `INTEN13` writer - Interrupt and wake-up enable control of TADC change over threshold."]
pub type INTEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN14` reader - Interrupt and wake-up enable control of ADC CH0 data below threshold."]
pub type INTEN14_R = crate::BitReader<bool>;
#[doc = "Field `INTEN14` writer - Interrupt and wake-up enable control of ADC CH0 data below threshold."]
pub type INTEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN15` reader - Interrupt and wake-up enable control of ADC CH0 data over threshold."]
pub type INTEN15_R = crate::BitReader<bool>;
#[doc = "Field `INTEN15` writer - Interrupt and wake-up enable control of ADC CH0 data over threshold."]
pub type INTEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN16` reader - Interrupt and wake-up enable control of ADC CH1 data below threshold."]
pub type INTEN16_R = crate::BitReader<bool>;
#[doc = "Field `INTEN16` writer - Interrupt and wake-up enable control of ADC CH1 data below threshold."]
pub type INTEN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN17` reader - Interrupt and wake-up enable control of ADC CH1 data over threshold."]
pub type INTEN17_R = crate::BitReader<bool>;
#[doc = "Field `INTEN17` writer - Interrupt and wake-up enable control of ADC CH1 data over threshold."]
pub type INTEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN18` reader - Interrupt and wake-up enable control of ADC CH2 data below threshold."]
pub type INTEN18_R = crate::BitReader<bool>;
#[doc = "Field `INTEN18` writer - Interrupt and wake-up enable control of ADC CH2 data below threshold."]
pub type INTEN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN19` reader - Interrupt and wake-up enable control of ADC CH2 data over threshold."]
pub type INTEN19_R = crate::BitReader<bool>;
#[doc = "Field `INTEN19` writer - Interrupt and wake-up enable control of ADC CH2 data over threshold."]
pub type INTEN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN20` reader - Interrupt and wake-up enable control of ADC CH3 data below threshold."]
pub type INTEN20_R = crate::BitReader<bool>;
#[doc = "Field `INTEN20` writer - Interrupt and wake-up enable control of ADC CH3 data below threshold."]
pub type INTEN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN21` reader - Interrupt and wake-up enable control of ADC CH3 data over threshold."]
pub type INTEN21_R = crate::BitReader<bool>;
#[doc = "Field `INTEN21` writer - Interrupt and wake-up enable control of ADC CH3 data over threshold."]
pub type INTEN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt enable control of manual ADC conversion done."]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable control of auto ADC conversion done."]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt and wake-up enable control of CMP1."]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt and wake-up enable control of CMP2."]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt and wake-up enable control of POWALARM."]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN7_R {
        INTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt and wake-up enable control of PWRDROP."]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt and wake-up enable control of POWLV."]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN10_R {
        INTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt and wake-up enable control of sleep mode entry, when VDCINDROP is 0."]
    #[inline(always)]
    pub fn inten11(&self) -> INTEN11_R {
        INTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt and wake-up enable control of ANA_REGx error."]
    #[inline(always)]
    pub fn inten12(&self) -> INTEN12_R {
        INTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt and wake-up enable control of TADC change over threshold."]
    #[inline(always)]
    pub fn inten13(&self) -> INTEN13_R {
        INTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt and wake-up enable control of ADC CH0 data below threshold."]
    #[inline(always)]
    pub fn inten14(&self) -> INTEN14_R {
        INTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt and wake-up enable control of ADC CH0 data over threshold."]
    #[inline(always)]
    pub fn inten15(&self) -> INTEN15_R {
        INTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt and wake-up enable control of ADC CH1 data below threshold."]
    #[inline(always)]
    pub fn inten16(&self) -> INTEN16_R {
        INTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt and wake-up enable control of ADC CH1 data over threshold."]
    #[inline(always)]
    pub fn inten17(&self) -> INTEN17_R {
        INTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt and wake-up enable control of ADC CH2 data below threshold."]
    #[inline(always)]
    pub fn inten18(&self) -> INTEN18_R {
        INTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt and wake-up enable control of ADC CH2 data over threshold."]
    #[inline(always)]
    pub fn inten19(&self) -> INTEN19_R {
        INTEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt and wake-up enable control of ADC CH3 data below threshold."]
    #[inline(always)]
    pub fn inten20(&self) -> INTEN20_R {
        INTEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt and wake-up enable control of ADC CH3 data over threshold."]
    #[inline(always)]
    pub fn inten21(&self) -> INTEN21_R {
        INTEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable control of manual ADC conversion done."]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN0_W<0> {
        INTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable control of auto ADC conversion done."]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN1_W<1> {
        INTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt and wake-up enable control of CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN2_W<2> {
        INTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt and wake-up enable control of CMP2."]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> INTEN3_W<3> {
        INTEN3_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt and wake-up enable control of POWALARM."]
    #[inline(always)]
    #[must_use]
    pub fn inten7(&mut self) -> INTEN7_W<7> {
        INTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt and wake-up enable control of PWRDROP."]
    #[inline(always)]
    #[must_use]
    pub fn inten8(&mut self) -> INTEN8_W<8> {
        INTEN8_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt and wake-up enable control of POWLV."]
    #[inline(always)]
    #[must_use]
    pub fn inten10(&mut self) -> INTEN10_W<10> {
        INTEN10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt and wake-up enable control of sleep mode entry, when VDCINDROP is 0."]
    #[inline(always)]
    #[must_use]
    pub fn inten11(&mut self) -> INTEN11_W<11> {
        INTEN11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt and wake-up enable control of ANA_REGx error."]
    #[inline(always)]
    #[must_use]
    pub fn inten12(&mut self) -> INTEN12_W<12> {
        INTEN12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt and wake-up enable control of TADC change over threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten13(&mut self) -> INTEN13_W<13> {
        INTEN13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt and wake-up enable control of ADC CH0 data below threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten14(&mut self) -> INTEN14_W<14> {
        INTEN14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt and wake-up enable control of ADC CH0 data over threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten15(&mut self) -> INTEN15_W<15> {
        INTEN15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt and wake-up enable control of ADC CH1 data below threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten16(&mut self) -> INTEN16_W<16> {
        INTEN16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt and wake-up enable control of ADC CH1 data over threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten17(&mut self) -> INTEN17_W<17> {
        INTEN17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt and wake-up enable control of ADC CH2 data below threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten18(&mut self) -> INTEN18_W<18> {
        INTEN18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt and wake-up enable control of ADC CH2 data over threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten19(&mut self) -> INTEN19_W<19> {
        INTEN19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt and wake-up enable control of ADC CH3 data below threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten20(&mut self) -> INTEN20_W<20> {
        INTEN20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt and wake-up enable control of ADC CH3 data over threshold."]
    #[inline(always)]
    #[must_use]
    pub fn inten21(&mut self) -> INTEN21_W<21> {
        INTEN21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `REGF` reader"]
pub struct R(crate::R<REGF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGF` writer"]
pub struct W(crate::W<REGF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGF_SPEC>;
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
impl From<crate::W<REGF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAT1DETEN` reader - Enabe connection from BAT1 to Comparator1‘s positive input."]
pub type BAT1DETEN_R = crate::BitReader<bool>;
#[doc = "Field `BAT1DETEN` writer - Enabe connection from BAT1 to Comparator1‘s positive input."]
pub type BAT1DETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `BATRTCDETEN` reader - Enabe connection from BATRTC to Comparator2‘s positive input."]
pub type BATRTCDETEN_R = crate::BitReader<bool>;
#[doc = "Field `BATRTCDETEN` writer - Enabe connection from BATRTC to Comparator2‘s positive input."]
pub type BATRTCDETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `AVCCOEN` reader - Power on AVCC_OUT pin."]
pub type AVCCOEN_R = crate::BitReader<bool>;
#[doc = "Field `AVCCOEN` writer - Power on AVCC_OUT pin."]
pub type AVCCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `ADTPDN` reader - Power up Tiny ADC."]
pub type ADTPDN_R = crate::BitReader<bool>;
#[doc = "Field `ADTPDN` writer - Power up Tiny ADC."]
pub type ADTPDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `ADTSEL` reader - Signal selection for ADT."]
pub type ADTSEL_R = crate::BitReader<bool>;
#[doc = "Field `ADTSEL` writer - Signal selection for ADT."]
pub type ADTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `ADTREF1SEL` reader - REF1 of ADT selection."]
pub type ADTREF1SEL_R = crate::BitReader<bool>;
#[doc = "Field `ADTREF1SEL` writer - REF1 of ADT selection."]
pub type ADTREF1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `ADTREF2SEL` reader - REF2 of ADT selection."]
pub type ADTREF2SEL_R = crate::BitReader<bool>;
#[doc = "Field `ADTREF2SEL` writer - REF2 of ADT selection."]
pub type ADTREF2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
#[doc = "Field `ADTREF3SEL` reader - REF3 of ADT selection."]
pub type ADTREF3SEL_R = crate::BitReader<bool>;
#[doc = "Field `ADTREF3SEL` writer - REF3 of ADT selection."]
pub type ADTREF3SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enabe connection from BAT1 to Comparator1‘s positive input."]
    #[inline(always)]
    pub fn bat1deten(&self) -> BAT1DETEN_R {
        BAT1DETEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enabe connection from BATRTC to Comparator2‘s positive input."]
    #[inline(always)]
    pub fn batrtcdeten(&self) -> BATRTCDETEN_R {
        BATRTCDETEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power on AVCC_OUT pin."]
    #[inline(always)]
    pub fn avccoen(&self) -> AVCCOEN_R {
        AVCCOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power up Tiny ADC."]
    #[inline(always)]
    pub fn adtpdn(&self) -> ADTPDN_R {
        ADTPDN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Signal selection for ADT."]
    #[inline(always)]
    pub fn adtsel(&self) -> ADTSEL_R {
        ADTSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REF1 of ADT selection."]
    #[inline(always)]
    pub fn adtref1sel(&self) -> ADTREF1SEL_R {
        ADTREF1SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REF2 of ADT selection."]
    #[inline(always)]
    pub fn adtref2sel(&self) -> ADTREF2SEL_R {
        ADTREF2SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REF3 of ADT selection."]
    #[inline(always)]
    pub fn adtref3sel(&self) -> ADTREF3SEL_R {
        ADTREF3SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enabe connection from BAT1 to Comparator1‘s positive input."]
    #[inline(always)]
    #[must_use]
    pub fn bat1deten(&mut self) -> BAT1DETEN_W<0> {
        BAT1DETEN_W::new(self)
    }
    #[doc = "Bit 1 - Enabe connection from BATRTC to Comparator2‘s positive input."]
    #[inline(always)]
    #[must_use]
    pub fn batrtcdeten(&mut self) -> BATRTCDETEN_W<1> {
        BATRTCDETEN_W::new(self)
    }
    #[doc = "Bit 2 - Power on AVCC_OUT pin."]
    #[inline(always)]
    #[must_use]
    pub fn avccoen(&mut self) -> AVCCOEN_W<2> {
        AVCCOEN_W::new(self)
    }
    #[doc = "Bit 3 - Power up Tiny ADC."]
    #[inline(always)]
    #[must_use]
    pub fn adtpdn(&mut self) -> ADTPDN_W<3> {
        ADTPDN_W::new(self)
    }
    #[doc = "Bit 4 - Signal selection for ADT."]
    #[inline(always)]
    #[must_use]
    pub fn adtsel(&mut self) -> ADTSEL_W<4> {
        ADTSEL_W::new(self)
    }
    #[doc = "Bit 5 - REF1 of ADT selection."]
    #[inline(always)]
    #[must_use]
    pub fn adtref1sel(&mut self) -> ADTREF1SEL_W<5> {
        ADTREF1SEL_W::new(self)
    }
    #[doc = "Bit 6 - REF2 of ADT selection."]
    #[inline(always)]
    #[must_use]
    pub fn adtref2sel(&mut self) -> ADTREF2SEL_W<6> {
        ADTREF2SEL_W::new(self)
    }
    #[doc = "Bit 7 - REF3 of ADT selection."]
    #[inline(always)]
    #[must_use]
    pub fn adtref3sel(&mut self) -> ADTREF3SEL_W<7> {
        ADTREF3SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regf](index.html) module"]
pub struct REGF_SPEC;
impl crate::RegisterSpec for REGF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regf::R](R) reader structure"]
impl crate::Readable for REGF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regf::W](W) writer structure"]
impl crate::Writable for REGF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGF to value 0"]
impl crate::Resettable for REGF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

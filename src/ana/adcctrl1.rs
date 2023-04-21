#[doc = "Register `ADCCTRL1` reader"]
pub struct R(crate::R<ADCCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTRL1` writer"]
pub struct W(crate::W<ADCCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTRL1_SPEC>;
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
impl From<crate::W<ADCCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER_THD0_EN` reader - LOWER_THD0 Enable"]
pub type LOWER_THD0_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD0_EN` writer - LOWER_THD0 Enable"]
pub type LOWER_THD0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `UPPER_THD0_EN` reader - UPPER_THD0 Enable"]
pub type UPPER_THD0_EN_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD0_EN` writer - UPPER_THD0 Enable"]
pub type UPPER_THD0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `LOWER_THD1_EN` reader - LOWER_THD1 Enable"]
pub type LOWER_THD1_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD1_EN` writer - LOWER_THD1 Enable"]
pub type LOWER_THD1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `UPPER_THD1_EN` reader - UPPER_THD1 Enable"]
pub type UPPER_THD1_EN_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD1_EN` writer - UPPER_THD1 Enable"]
pub type UPPER_THD1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `LOWER_THD2_EN` reader - LOWER_THD2 Enable"]
pub type LOWER_THD2_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD2_EN` writer - LOWER_THD2 Enable"]
pub type LOWER_THD2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `UPPER_THD2_EN` reader - UPPER_THD2 Enable"]
pub type UPPER_THD2_EN_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD2_EN` writer - UPPER_THD2 Enable"]
pub type UPPER_THD2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `LOWER_THD3_EN` reader - LOWER_THD3 Enable"]
pub type LOWER_THD3_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD3_EN` writer - LOWER_THD3 Enable"]
pub type LOWER_THD3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `UPPER_THD3_EN` reader - UPPER_THD3 Enable"]
pub type UPPER_THD3_EN_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD3_EN` writer - UPPER_THD3 Enable"]
pub type UPPER_THD3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL1_SPEC, bool, O>;
#[doc = "Field `RESDIV_CHx` reader - BITy Control CHx RESDIV enable. y=31~16, x=15~0."]
pub type RESDIV_CHX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESDIV_CHx` writer - BITy Control CHx RESDIV enable. y=31~16, x=15~0."]
pub type RESDIV_CHX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCCTRL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 8 - LOWER_THD0 Enable"]
    #[inline(always)]
    pub fn lower_thd0_en(&self) -> LOWER_THD0_EN_R {
        LOWER_THD0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UPPER_THD0 Enable"]
    #[inline(always)]
    pub fn upper_thd0_en(&self) -> UPPER_THD0_EN_R {
        UPPER_THD0_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LOWER_THD1 Enable"]
    #[inline(always)]
    pub fn lower_thd1_en(&self) -> LOWER_THD1_EN_R {
        LOWER_THD1_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UPPER_THD1 Enable"]
    #[inline(always)]
    pub fn upper_thd1_en(&self) -> UPPER_THD1_EN_R {
        UPPER_THD1_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LOWER_THD2 Enable"]
    #[inline(always)]
    pub fn lower_thd2_en(&self) -> LOWER_THD2_EN_R {
        LOWER_THD2_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UPPER_THD2 Enable"]
    #[inline(always)]
    pub fn upper_thd2_en(&self) -> UPPER_THD2_EN_R {
        UPPER_THD2_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LOWER_THD3 Enable"]
    #[inline(always)]
    pub fn lower_thd3_en(&self) -> LOWER_THD3_EN_R {
        LOWER_THD3_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - UPPER_THD3 Enable"]
    #[inline(always)]
    pub fn upper_thd3_en(&self) -> UPPER_THD3_EN_R {
        UPPER_THD3_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - BITy Control CHx RESDIV enable. y=31~16, x=15~0."]
    #[inline(always)]
    pub fn resdiv_chx(&self) -> RESDIV_CHX_R {
        RESDIV_CHX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 8 - LOWER_THD0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd0_en(&mut self) -> LOWER_THD0_EN_W<8> {
        LOWER_THD0_EN_W::new(self)
    }
    #[doc = "Bit 9 - UPPER_THD0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd0_en(&mut self) -> UPPER_THD0_EN_W<9> {
        UPPER_THD0_EN_W::new(self)
    }
    #[doc = "Bit 10 - LOWER_THD1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd1_en(&mut self) -> LOWER_THD1_EN_W<10> {
        LOWER_THD1_EN_W::new(self)
    }
    #[doc = "Bit 11 - UPPER_THD1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd1_en(&mut self) -> UPPER_THD1_EN_W<11> {
        UPPER_THD1_EN_W::new(self)
    }
    #[doc = "Bit 12 - LOWER_THD2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd2_en(&mut self) -> LOWER_THD2_EN_W<12> {
        LOWER_THD2_EN_W::new(self)
    }
    #[doc = "Bit 13 - UPPER_THD2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd2_en(&mut self) -> UPPER_THD2_EN_W<13> {
        UPPER_THD2_EN_W::new(self)
    }
    #[doc = "Bit 14 - LOWER_THD3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd3_en(&mut self) -> LOWER_THD3_EN_W<14> {
        LOWER_THD3_EN_W::new(self)
    }
    #[doc = "Bit 15 - UPPER_THD3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd3_en(&mut self) -> UPPER_THD3_EN_W<15> {
        UPPER_THD3_EN_W::new(self)
    }
    #[doc = "Bits 16:31 - BITy Control CHx RESDIV enable. y=31~16, x=15~0."]
    #[inline(always)]
    #[must_use]
    pub fn resdiv_chx(&mut self) -> RESDIV_CHX_W<16> {
        RESDIV_CHX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANA_ADCCTRL1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctrl1](index.html) module"]
pub struct ADCCTRL1_SPEC;
impl crate::RegisterSpec for ADCCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcctrl1::R](R) reader structure"]
impl crate::Readable for ADCCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctrl1::W](W) writer structure"]
impl crate::Writable for ADCCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTRL1 to value 0xc0"]
impl crate::Resettable for ADCCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}

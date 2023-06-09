#[doc = "Register `WKUCNT` reader"]
pub struct R(crate::R<WKUCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUCNT` writer"]
pub struct W(crate::W<WKUCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUCNT_SPEC>;
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
impl From<crate::W<WKUCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUCNT` reader - This register is used to control the 32K counter wake-up function."]
pub type WKUCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WKUCNT` writer - This register is used to control the 32K counter wake-up function."]
pub type WKUCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUCNT_SPEC, u32, u32, 24, O>;
#[doc = "Field `CNTSEL` reader - This is register is used to set the counter clock of WKUCNT."]
pub type CNTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTSEL` writer - This is register is used to set the counter clock of WKUCNT."]
pub type CNTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUCNT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:23 - This register is used to control the 32K counter wake-up function."]
    #[inline(always)]
    pub fn wkucnt(&self) -> WKUCNT_R {
        WKUCNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - This is register is used to set the counter clock of WKUCNT."]
    #[inline(always)]
    pub fn cntsel(&self) -> CNTSEL_R {
        CNTSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to control the 32K counter wake-up function."]
    #[inline(always)]
    #[must_use]
    pub fn wkucnt(&mut self) -> WKUCNT_W<0> {
        WKUCNT_W::new(self)
    }
    #[doc = "Bits 24:25 - This is register is used to set the counter clock of WKUCNT."]
    #[inline(always)]
    #[must_use]
    pub fn cntsel(&mut self) -> CNTSEL_W<24> {
        CNTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC wake-up counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkucnt](index.html) module"]
pub struct WKUCNT_SPEC;
impl crate::RegisterSpec for WKUCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkucnt::R](R) reader structure"]
impl crate::Readable for WKUCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkucnt::W](W) writer structure"]
impl crate::Writable for WKUCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUCNT to value 0"]
impl crate::Resettable for WKUCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

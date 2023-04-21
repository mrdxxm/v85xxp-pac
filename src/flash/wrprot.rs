#[doc = "Register `WRPROT` reader"]
pub struct R(crate::R<WRPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPROT` writer"]
pub struct W(crate::W<WRPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPROT_SPEC>;
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
impl From<crate::W<WRPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRPORT` reader - This register is used to protect specific region of flash from erase or program operation."]
pub type WRPORT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRPORT` writer - This register is used to protect specific region of flash from erase or program operation."]
pub type WRPORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPROT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register is used to protect specific region of flash from erase or program operation."]
    #[inline(always)]
    pub fn wrport(&self) -> WRPORT_R {
        WRPORT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to protect specific region of flash from erase or program operation."]
    #[inline(always)]
    #[must_use]
    pub fn wrport(&mut self) -> WRPORT_W<0> {
        WRPORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash write protect control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprot](index.html) module"]
pub struct WRPROT_SPEC;
impl crate::RegisterSpec for WRPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrprot::R](R) reader structure"]
impl crate::Readable for WRPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrprot::W](W) writer structure"]
impl crate::Writable for WRPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPROT to value 0"]
impl crate::Resettable for WRPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

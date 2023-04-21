#[doc = "Register `BAUDDIV` reader"]
pub struct R(crate::R<BAUDDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDDIV` writer"]
pub struct W(crate::W<BAUDDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDDIV_SPEC>;
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
impl From<crate::W<BAUDDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUDDIV` reader - Baud rate divider register."]
pub type BAUDDIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BAUDDIV` writer - Baud rate divider register."]
pub type BAUDDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUDDIV_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Baud rate divider register."]
    #[inline(always)]
    pub fn bauddiv(&self) -> BAUDDIV_R {
        BAUDDIV_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Baud rate divider register."]
    #[inline(always)]
    #[must_use]
    pub fn bauddiv(&mut self) -> BAUDDIV_W<0> {
        BAUDDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART baud rate divide register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bauddiv](index.html) module"]
pub struct BAUDDIV_SPEC;
impl crate::RegisterSpec for BAUDDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bauddiv::R](R) reader structure"]
impl crate::Readable for BAUDDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bauddiv::W](W) writer structure"]
impl crate::Writable for BAUDDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUDDIV to value 0"]
impl crate::Resettable for BAUDDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

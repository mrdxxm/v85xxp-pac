#[doc = "Register `PGADDR` reader"]
pub struct R(crate::R<PGADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGADDR` writer"]
pub struct W(crate::W<PGADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGADDR_SPEC>;
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
impl From<crate::W<PGADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGADDR` reader - This register is used to control the program address before doing program."]
pub type PGADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PGADDR` writer - This register is used to control the program address before doing program."]
pub type PGADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGADDR_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - This register is used to control the program address before doing program."]
    #[inline(always)]
    pub fn pgaddr(&self) -> PGADDR_R {
        PGADDR_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - This register is used to control the program address before doing program."]
    #[inline(always)]
    #[must_use]
    pub fn pgaddr(&mut self) -> PGADDR_W<0> {
        PGADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH program address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgaddr](index.html) module"]
pub struct PGADDR_SPEC;
impl crate::RegisterSpec for PGADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgaddr::R](R) reader structure"]
impl crate::Readable for PGADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgaddr::W](W) writer structure"]
impl crate::Writable for PGADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGADDR to value 0"]
impl crate::Resettable for PGADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

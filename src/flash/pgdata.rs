#[doc = "Register `PGDATA` reader"]
pub struct R(crate::R<PGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGDATA` writer"]
pub struct W(crate::W<PGDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGDATA_SPEC>;
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
impl From<crate::W<PGDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGDATA` reader - This register is used to control the program data."]
pub type PGDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PGDATA` writer - This register is used to control the program data."]
pub type PGDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register is used to control the program data."]
    #[inline(always)]
    pub fn pgdata(&self) -> PGDATA_R {
        PGDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to control the program data."]
    #[inline(always)]
    #[must_use]
    pub fn pgdata(&mut self) -> PGDATA_W<0> {
        PGDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH program word data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgdata](index.html) module"]
pub struct PGDATA_SPEC;
impl crate::RegisterSpec for PGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgdata::R](R) reader structure"]
impl crate::Readable for PGDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgdata::W](W) writer structure"]
impl crate::Writable for PGDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGDATA to value 0"]
impl crate::Resettable for PGDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

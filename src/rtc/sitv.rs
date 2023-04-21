#[doc = "Register `SITV` reader"]
pub struct R(crate::R<SITV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SITV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SITV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SITV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SITV` writer"]
pub struct W(crate::W<SITV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SITV_SPEC>;
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
impl From<crate::W<SITV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SITV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SITV` reader - Multi second wake-up interval control register."]
pub type SITV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SITV` writer - Multi second wake-up interval control register."]
pub type SITV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SITV_SPEC, u8, u8, 6, O>;
#[doc = "Field `SITVEN` reader - Multi Second interval enable register."]
pub type SITVEN_R = crate::BitReader<bool>;
#[doc = "Field `SITVEN` writer - Multi Second interval enable register."]
pub type SITVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SITV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Multi second wake-up interval control register."]
    #[inline(always)]
    pub fn sitv(&self) -> SITV_R {
        SITV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Multi Second interval enable register."]
    #[inline(always)]
    pub fn sitven(&self) -> SITVEN_R {
        SITVEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Multi second wake-up interval control register."]
    #[inline(always)]
    #[must_use]
    pub fn sitv(&mut self) -> SITV_W<0> {
        SITV_W::new(self)
    }
    #[doc = "Bit 6 - Multi Second interval enable register."]
    #[inline(always)]
    #[must_use]
    pub fn sitven(&mut self) -> SITVEN_W<6> {
        SITVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC wake-up second interval control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sitv](index.html) module"]
pub struct SITV_SPEC;
impl crate::RegisterSpec for SITV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sitv::R](R) reader structure"]
impl crate::Readable for SITV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sitv::W](W) writer structure"]
impl crate::Writable for SITV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SITV to value 0"]
impl crate::Resettable for SITV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

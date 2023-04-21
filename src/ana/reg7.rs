#[doc = "Register `REG7` reader"]
pub struct R(crate::R<REG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG7` writer"]
pub struct W(crate::W<REG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG7_SPEC>;
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
impl From<crate::W<REG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDCINHYSSEL` reader - VDCIN comparator hysteresis."]
pub type VDCINHYSSEL_R = crate::BitReader<bool>;
#[doc = "Field `VDCINHYSSEL` writer - VDCIN comparator hysteresis."]
pub type VDCINHYSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG7_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - VDCIN comparator hysteresis."]
    #[inline(always)]
    pub fn vdcinhyssel(&self) -> VDCINHYSSEL_R {
        VDCINHYSSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - VDCIN comparator hysteresis."]
    #[inline(always)]
    #[must_use]
    pub fn vdcinhyssel(&mut self) -> VDCINHYSSEL_W<2> {
        VDCINHYSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg7](index.html) module"]
pub struct REG7_SPEC;
impl crate::RegisterSpec for REG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg7::R](R) reader structure"]
impl crate::Readable for REG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg7::W](W) writer structure"]
impl crate::Writable for REG7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG7 to value 0"]
impl crate::Resettable for REG7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

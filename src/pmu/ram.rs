#[doc = "Register `RAM[%s]` reader"]
pub struct R(crate::R<RAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM[%s]` writer"]
pub struct W(crate::W<RAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_SPEC>;
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
impl From<crate::W<RAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM` reader - There is a 256 bytes (64x32) SRAM embedded in the PMU controller. This RAM can keep data during deep-sleep mode. Only word access is allowed to these ports."]
pub type RAM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RAM` writer - There is a 256 bytes (64x32) SRAM embedded in the PMU controller. This RAM can keep data during deep-sleep mode. Only word access is allowed to these ports."]
pub type RAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - There is a 256 bytes (64x32) SRAM embedded in the PMU controller. This RAM can keep data during deep-sleep mode. Only word access is allowed to these ports."]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - There is a 256 bytes (64x32) SRAM embedded in the PMU controller. This RAM can keep data during deep-sleep mode. Only word access is allowed to these ports."]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RAM_W<0> {
        RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU 32 bits Retention RAM x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram](index.html) module"]
pub struct RAM_SPEC;
impl crate::RegisterSpec for RAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram::R](R) reader structure"]
impl crate::Readable for RAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram::W](W) writer structure"]
impl crate::Writable for RAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM[%s]
to value 0"]
impl crate::Resettable for RAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

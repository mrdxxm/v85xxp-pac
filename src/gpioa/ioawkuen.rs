#[doc = "Register `IOAWKUEN` reader"]
pub struct R(crate::R<IOAWKUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOAWKUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOAWKUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOAWKUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOAWKUEN` writer"]
pub struct W(crate::W<IOAWKUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOAWKUEN_SPEC>;
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
impl From<crate::W<IOAWKUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOAWKUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUEN` reader - Every 2 bits control the IOA' s wake up function. Bit \\[1:0\\]: IOA0 Bit \\[3:2\\]: IOA1 ... Bit \\[31:30\\]: IOA15"]
pub type WKUEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WKUEN` writer - Every 2 bits control the IOA' s wake up function. Bit \\[1:0\\]: IOA0 Bit \\[3:2\\]: IOA1 ... Bit \\[31:30\\]: IOA15"]
pub type WKUEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOAWKUEN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Every 2 bits control the IOA' s wake up function. Bit \\[1:0\\]: IOA0 Bit \\[3:2\\]: IOA1 ... Bit \\[31:30\\]: IOA15"]
    #[inline(always)]
    pub fn wkuen(&self) -> WKUEN_R {
        WKUEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Every 2 bits control the IOA' s wake up function. Bit \\[1:0\\]: IOA0 Bit \\[3:2\\]: IOA1 ... Bit \\[31:30\\]: IOA15"]
    #[inline(always)]
    #[must_use]
    pub fn wkuen(&mut self) -> WKUEN_W<0> {
        WKUEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOA wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioawkuen](index.html) module"]
pub struct IOAWKUEN_SPEC;
impl crate::RegisterSpec for IOAWKUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioawkuen::R](R) reader structure"]
impl crate::Readable for IOAWKUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioawkuen::W](W) writer structure"]
impl crate::Writable for IOAWKUEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOAWKUEN to value 0"]
impl crate::Resettable for IOAWKUEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

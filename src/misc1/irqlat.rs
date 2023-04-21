#[doc = "Register `IRQLAT` reader"]
pub struct R(crate::R<IRQLAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQLAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQLAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQLAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQLAT` writer"]
pub struct W(crate::W<IRQLAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQLAT_SPEC>;
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
impl From<crate::W<IRQLAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQLAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQLAT` reader - This register is used to control the Cortex M0 IRQ latency."]
pub type IRQLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQLAT` writer - This register is used to control the Cortex M0 IRQ latency."]
pub type IRQLAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQLAT_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCKRESET` reader - This register is used to control if the lockup will issue a system reset."]
pub type LOCKRESET_R = crate::BitReader<bool>;
#[doc = "Field `LOCKRESET` writer - This register is used to control if the lockup will issue a system reset."]
pub type LOCKRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQLAT_SPEC, bool, O>;
#[doc = "Field `NOHARDFAULT` reader - This register is used to disable the hard fault generation to CPU."]
pub type NOHARDFAULT_R = crate::BitReader<bool>;
#[doc = "Field `NOHARDFAULT` writer - This register is used to disable the hard fault generation to CPU."]
pub type NOHARDFAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQLAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to control the Cortex M0 IRQ latency."]
    #[inline(always)]
    pub fn irqlat(&self) -> IRQLAT_R {
        IRQLAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This register is used to control if the lockup will issue a system reset."]
    #[inline(always)]
    pub fn lockreset(&self) -> LOCKRESET_R {
        LOCKRESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This register is used to disable the hard fault generation to CPU."]
    #[inline(always)]
    pub fn nohardfault(&self) -> NOHARDFAULT_R {
        NOHARDFAULT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to control the Cortex M0 IRQ latency."]
    #[inline(always)]
    #[must_use]
    pub fn irqlat(&mut self) -> IRQLAT_W<0> {
        IRQLAT_W::new(self)
    }
    #[doc = "Bit 8 - This register is used to control if the lockup will issue a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn lockreset(&mut self) -> LOCKRESET_W<8> {
        LOCKRESET_W::new(self)
    }
    #[doc = "Bit 9 - This register is used to disable the hard fault generation to CPU."]
    #[inline(always)]
    #[must_use]
    pub fn nohardfault(&mut self) -> NOHARDFAULT_W<9> {
        NOHARDFAULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cortex M0 IRQ latency control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqlat](index.html) module"]
pub struct IRQLAT_SPEC;
impl crate::RegisterSpec for IRQLAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqlat::R](R) reader structure"]
impl crate::Readable for IRQLAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqlat::W](W) writer structure"]
impl crate::Writable for IRQLAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQLAT to value 0"]
impl crate::Resettable for IRQLAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

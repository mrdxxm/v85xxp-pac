#[doc = "Register `SRAMINT` reader"]
pub struct R(crate::R<SRAMINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMINT` writer"]
pub struct W(crate::W<SRAMINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMINT_SPEC>;
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
impl From<crate::W<SRAMINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERR` reader - This bit indicates that a SRAM parity error is happened during the SRAM read process."]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - This bit indicates that a SRAM parity error is happened during the SRAM read process."]
pub type PERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRAMINT_SPEC, bool, O>;
#[doc = "Field `HIAL` reader - This bit indicates that an invalid align access on AHB bus is occurred."]
pub type HIAL_R = crate::BitReader<bool>;
#[doc = "Field `HIAL` writer - This bit indicates that an invalid align access on AHB bus is occurred."]
pub type HIAL_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRAMINT_SPEC, bool, O>;
#[doc = "Field `HIAC` reader - This bit indicates that an invalid address access on AHB bus is occurred."]
pub type HIAC_R = crate::BitReader<bool>;
#[doc = "Field `HIAC` writer - This bit indicates that an invalid address access on AHB bus is occurred."]
pub type HIAC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRAMINT_SPEC, bool, O>;
#[doc = "Field `PIAC` reader - This bit indicates that an invalid address access on APB bus is occurred."]
pub type PIAC_R = crate::BitReader<bool>;
#[doc = "Field `PIAC` writer - This bit indicates that an invalid address access on APB bus is occurred."]
pub type PIAC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRAMINT_SPEC, bool, O>;
#[doc = "Field `LOCKUP` reader - This bit indicates the CM0 lockup has happened."]
pub type LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP` writer - This bit indicates the CM0 lockup has happened."]
pub type LOCKUP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRAMINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit indicates that a SRAM parity error is happened during the SRAM read process."]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that an invalid align access on AHB bus is occurred."]
    #[inline(always)]
    pub fn hial(&self) -> HIAL_R {
        HIAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that an invalid address access on AHB bus is occurred."]
    #[inline(always)]
    pub fn hiac(&self) -> HIAC_R {
        HIAC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that an invalid address access on APB bus is occurred."]
    #[inline(always)]
    pub fn piac(&self) -> PIAC_R {
        PIAC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the CM0 lockup has happened."]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates that a SRAM parity error is happened during the SRAM read process."]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<0> {
        PERR_W::new(self)
    }
    #[doc = "Bit 1 - This bit indicates that an invalid align access on AHB bus is occurred."]
    #[inline(always)]
    #[must_use]
    pub fn hial(&mut self) -> HIAL_W<1> {
        HIAL_W::new(self)
    }
    #[doc = "Bit 2 - This bit indicates that an invalid address access on AHB bus is occurred."]
    #[inline(always)]
    #[must_use]
    pub fn hiac(&mut self) -> HIAC_W<2> {
        HIAC_W::new(self)
    }
    #[doc = "Bit 3 - This bit indicates that an invalid address access on APB bus is occurred."]
    #[inline(always)]
    #[must_use]
    pub fn piac(&mut self) -> PIAC_W<3> {
        PIAC_W::new(self)
    }
    #[doc = "Bit 4 - This bit indicates the CM0 lockup has happened."]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LOCKUP_W<4> {
        LOCKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Parity Error Interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramint](index.html) module"]
pub struct SRAMINT_SPEC;
impl crate::RegisterSpec for SRAMINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramint::R](R) reader structure"]
impl crate::Readable for SRAMINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramint::W](W) writer structure"]
impl crate::Writable for SRAMINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
}
#[doc = "`reset()` method sets SRAMINT to value 0"]
impl crate::Resettable for SRAMINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

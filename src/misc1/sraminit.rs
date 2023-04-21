#[doc = "Register `SRAMINIT` reader"]
pub struct R(crate::R<SRAMINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMINIT` writer"]
pub struct W(crate::W<SRAMINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMINIT_SPEC>;
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
impl From<crate::W<SRAMINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN` reader - Parity check enable register."]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - Parity check enable register."]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMINIT_SPEC, bool, O>;
#[doc = "Field `PERRIE` reader - SRAM parity error NMI enable register."]
pub type PERRIE_R = crate::BitReader<bool>;
#[doc = "Field `PERRIE` writer - SRAM parity error NMI enable register."]
pub type PERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMINIT_SPEC, bool, O>;
#[doc = "Field `INIT` reader - SRAM initialize register."]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - SRAM initialize register."]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMINIT_SPEC, bool, O>;
#[doc = "Field `HIACIE` reader - AHB invalid address access NMI enable register."]
pub type HIACIE_R = crate::BitReader<bool>;
#[doc = "Field `HIACIE` writer - AHB invalid address access NMI enable register."]
pub type HIACIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMINIT_SPEC, bool, O>;
#[doc = "Field `PIACIE` reader - APB invalid address access NMI enable register."]
pub type PIACIE_R = crate::BitReader<bool>;
#[doc = "Field `PIACIE` writer - APB invalid address access NMI enable register."]
pub type PIACIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMINIT_SPEC, bool, O>;
#[doc = "Field `LOCKIE` reader - CM0 lockup NMI enable register."]
pub type LOCKIE_R = crate::BitReader<bool>;
#[doc = "Field `LOCKIE` writer - CM0 lockup NMI enable register."]
pub type LOCKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMINIT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Parity check enable register."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity error NMI enable register."]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM initialize register."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB invalid address access NMI enable register."]
    #[inline(always)]
    pub fn hiacie(&self) -> HIACIE_R {
        HIACIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APB invalid address access NMI enable register."]
    #[inline(always)]
    pub fn piacie(&self) -> PIACIE_R {
        PIACIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CM0 lockup NMI enable register."]
    #[inline(always)]
    pub fn lockie(&self) -> LOCKIE_R {
        LOCKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity check enable register."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<0> {
        PEN_W::new(self)
    }
    #[doc = "Bit 1 - SRAM parity error NMI enable register."]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<1> {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 2 - SRAM initialize register."]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<2> {
        INIT_W::new(self)
    }
    #[doc = "Bit 5 - AHB invalid address access NMI enable register."]
    #[inline(always)]
    #[must_use]
    pub fn hiacie(&mut self) -> HIACIE_W<5> {
        HIACIE_W::new(self)
    }
    #[doc = "Bit 6 - APB invalid address access NMI enable register."]
    #[inline(always)]
    #[must_use]
    pub fn piacie(&mut self) -> PIACIE_W<6> {
        PIACIE_W::new(self)
    }
    #[doc = "Bit 7 - CM0 lockup NMI enable register."]
    #[inline(always)]
    #[must_use]
    pub fn lockie(&mut self) -> LOCKIE_W<7> {
        LOCKIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM initialize register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sraminit](index.html) module"]
pub struct SRAMINIT_SPEC;
impl crate::RegisterSpec for SRAMINIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sraminit::R](R) reader structure"]
impl crate::Readable for SRAMINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sraminit::W](W) writer structure"]
impl crate::Writable for SRAMINIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMINIT to value 0"]
impl crate::Resettable for SRAMINIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

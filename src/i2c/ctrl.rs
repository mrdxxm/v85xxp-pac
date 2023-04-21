#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR0` reader - Clock rate bit 0."]
pub type CR0_R = crate::BitReader<bool>;
#[doc = "Field `CR0` writer - Clock rate bit 0."]
pub type CR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CR1` reader - Clock rate bit 1."]
pub type CR1_R = crate::BitReader<bool>;
#[doc = "Field `CR1` writer - Clock rate bit 1."]
pub type CR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AA` reader - Assert Acknowledge Flag."]
pub type AA_R = crate::BitReader<bool>;
#[doc = "Field `AA` writer - Assert Acknowledge Flag."]
pub type AA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SI` reader - Serial Interrupt Flag."]
pub type SI_R = crate::BitReader<bool>;
#[doc = "Field `SI` writer - Serial Interrupt Flag."]
pub type SI_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STO` reader - STOP Flag."]
pub type STO_R = crate::BitReader<bool>;
#[doc = "Field `STO` writer - STOP Flag."]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STA` reader - START Flag."]
pub type STA_R = crate::BitReader<bool>;
#[doc = "Field `STA` writer - START Flag."]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EN` reader - I2C enable bit."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - I2C enable bit."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CR2` reader - Clock rate bit 2."]
pub type CR2_R = crate::BitReader<bool>;
#[doc = "Field `CR2` writer - Clock rate bit 2."]
pub type CR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock rate bit 0."]
    #[inline(always)]
    pub fn cr0(&self) -> CR0_R {
        CR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock rate bit 1."]
    #[inline(always)]
    pub fn cr1(&self) -> CR1_R {
        CR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Assert Acknowledge Flag."]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Serial Interrupt Flag."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP Flag."]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START Flag."]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C enable bit."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock rate bit 2."]
    #[inline(always)]
    pub fn cr2(&self) -> CR2_R {
        CR2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock rate bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<0> {
        CR0_W::new(self)
    }
    #[doc = "Bit 1 - Clock rate bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<1> {
        CR1_W::new(self)
    }
    #[doc = "Bit 2 - Assert Acknowledge Flag."]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<2> {
        AA_W::new(self)
    }
    #[doc = "Bit 3 - Serial Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn si(&mut self) -> SI_W<3> {
        SI_W::new(self)
    }
    #[doc = "Bit 4 - STOP Flag."]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<4> {
        STO_W::new(self)
    }
    #[doc = "Bit 5 - START Flag."]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<5> {
        STA_W::new(self)
    }
    #[doc = "Bit 6 - I2C enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<6> {
        EN_W::new(self)
    }
    #[doc = "Bit 7 - Clock rate bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<7> {
        CR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C control/status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x08;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

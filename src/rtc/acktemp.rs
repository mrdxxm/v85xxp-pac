#[doc = "Register `ACKTEMP` reader"]
pub struct R(crate::R<ACKTEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACKTEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACKTEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACKTEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACKTEMP` writer"]
pub struct W(crate::W<ACKTEMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACKTEMP_SPEC>;
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
impl From<crate::W<ACKTEMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACKTEMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KTEMP1` reader - This register is used to control the section 1 temperature."]
pub type KTEMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KTEMP1` writer - This register is used to control the section 1 temperature."]
pub type KTEMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACKTEMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `KTEMP2` reader - This register is used to control the section 2 temperature."]
pub type KTEMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KTEMP2` writer - This register is used to control the section 2 temperature."]
pub type KTEMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACKTEMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `KTEMP3` reader - This register is used to control the section 3 temperature."]
pub type KTEMP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KTEMP3` writer - This register is used to control the section 3 temperature."]
pub type KTEMP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACKTEMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `KTEMP4` reader - This register is used to control the section 4 temperature."]
pub type KTEMP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KTEMP4` writer - This register is used to control the section 4 temperature."]
pub type KTEMP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACKTEMP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to control the section 1 temperature."]
    #[inline(always)]
    pub fn ktemp1(&self) -> KTEMP1_R {
        KTEMP1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to control the section 2 temperature."]
    #[inline(always)]
    pub fn ktemp2(&self) -> KTEMP2_R {
        KTEMP2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register is used to control the section 3 temperature."]
    #[inline(always)]
    pub fn ktemp3(&self) -> KTEMP3_R {
        KTEMP3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register is used to control the section 4 temperature."]
    #[inline(always)]
    pub fn ktemp4(&self) -> KTEMP4_R {
        KTEMP4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to control the section 1 temperature."]
    #[inline(always)]
    #[must_use]
    pub fn ktemp1(&mut self) -> KTEMP1_W<0> {
        KTEMP1_W::new(self)
    }
    #[doc = "Bits 8:15 - This register is used to control the section 2 temperature."]
    #[inline(always)]
    #[must_use]
    pub fn ktemp2(&mut self) -> KTEMP2_W<8> {
        KTEMP2_W::new(self)
    }
    #[doc = "Bits 16:23 - This register is used to control the section 3 temperature."]
    #[inline(always)]
    #[must_use]
    pub fn ktemp3(&mut self) -> KTEMP3_W<16> {
        KTEMP3_W::new(self)
    }
    #[doc = "Bits 24:31 - This register is used to control the section 4 temperature."]
    #[inline(always)]
    #[must_use]
    pub fn ktemp4(&mut self) -> KTEMP4_W<24> {
        KTEMP4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC auto-calibration k temperature section control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acktemp](index.html) module"]
pub struct ACKTEMP_SPEC;
impl crate::RegisterSpec for ACKTEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acktemp::R](R) reader structure"]
impl crate::Readable for ACKTEMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acktemp::W](W) writer structure"]
impl crate::Writable for ACKTEMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACKTEMP to value 0x3c28_00ec"]
impl crate::Resettable for ACKTEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c28_00ec;
}

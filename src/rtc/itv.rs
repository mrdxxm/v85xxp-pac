#[doc = "Register `ITV` reader"]
pub struct R(crate::R<ITV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITV` writer"]
pub struct W(crate::W<ITV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITV_SPEC>;
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
impl From<crate::W<ITV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITV` reader - This register is used to control wake-up and interrupt interval of RTC."]
pub type ITV_R = crate::FieldReader<u8, ITV_A>;
#[doc = "This register is used to control wake-up and interrupt interval of RTC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITV_A {
    #[doc = "0: `0`"]
    ONE_SECOND = 0,
    #[doc = "1: `1`"]
    ONE_MINUTE = 1,
    #[doc = "2: `10`"]
    ONE_HOUR = 2,
    #[doc = "3: `11`"]
    ONE_DAY = 3,
    #[doc = "4: `100`"]
    EVERY_500MS = 4,
    #[doc = "5: `101`"]
    EVERY_250MS = 5,
    #[doc = "6: `110`"]
    EVERY_125MS = 6,
    #[doc = "7: `111`"]
    EVERY_62_5MS = 7,
}
impl From<ITV_A> for u8 {
    #[inline(always)]
    fn from(variant: ITV_A) -> Self {
        variant as _
    }
}
impl ITV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITV_A {
        match self.bits {
            0 => ITV_A::ONE_SECOND,
            1 => ITV_A::ONE_MINUTE,
            2 => ITV_A::ONE_HOUR,
            3 => ITV_A::ONE_DAY,
            4 => ITV_A::EVERY_500MS,
            5 => ITV_A::EVERY_250MS,
            6 => ITV_A::EVERY_125MS,
            7 => ITV_A::EVERY_62_5MS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SECOND`"]
    #[inline(always)]
    pub fn is_one_second(&self) -> bool {
        *self == ITV_A::ONE_SECOND
    }
    #[doc = "Checks if the value of the field is `ONE_MINUTE`"]
    #[inline(always)]
    pub fn is_one_minute(&self) -> bool {
        *self == ITV_A::ONE_MINUTE
    }
    #[doc = "Checks if the value of the field is `ONE_HOUR`"]
    #[inline(always)]
    pub fn is_one_hour(&self) -> bool {
        *self == ITV_A::ONE_HOUR
    }
    #[doc = "Checks if the value of the field is `ONE_DAY`"]
    #[inline(always)]
    pub fn is_one_day(&self) -> bool {
        *self == ITV_A::ONE_DAY
    }
    #[doc = "Checks if the value of the field is `EVERY_500MS`"]
    #[inline(always)]
    pub fn is_every_500ms(&self) -> bool {
        *self == ITV_A::EVERY_500MS
    }
    #[doc = "Checks if the value of the field is `EVERY_250MS`"]
    #[inline(always)]
    pub fn is_every_250ms(&self) -> bool {
        *self == ITV_A::EVERY_250MS
    }
    #[doc = "Checks if the value of the field is `EVERY_125MS`"]
    #[inline(always)]
    pub fn is_every_125ms(&self) -> bool {
        *self == ITV_A::EVERY_125MS
    }
    #[doc = "Checks if the value of the field is `EVERY_62_5MS`"]
    #[inline(always)]
    pub fn is_every_62_5ms(&self) -> bool {
        *self == ITV_A::EVERY_62_5MS
    }
}
#[doc = "Field `ITV` writer - This register is used to control wake-up and interrupt interval of RTC."]
pub type ITV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ITV_SPEC, u8, ITV_A, 3, O>;
impl<'a, const O: u8> ITV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_second(self) -> &'a mut W {
        self.variant(ITV_A::ONE_SECOND)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn one_minute(self) -> &'a mut W {
        self.variant(ITV_A::ONE_MINUTE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn one_hour(self) -> &'a mut W {
        self.variant(ITV_A::ONE_HOUR)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn one_day(self) -> &'a mut W {
        self.variant(ITV_A::ONE_DAY)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn every_500ms(self) -> &'a mut W {
        self.variant(ITV_A::EVERY_500MS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn every_250ms(self) -> &'a mut W {
        self.variant(ITV_A::EVERY_250MS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn every_125ms(self) -> &'a mut W {
        self.variant(ITV_A::EVERY_125MS)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn every_62_5ms(self) -> &'a mut W {
        self.variant(ITV_A::EVERY_62_5MS)
    }
}
impl R {
    #[doc = "Bits 0:2 - This register is used to control wake-up and interrupt interval of RTC."]
    #[inline(always)]
    pub fn itv(&self) -> ITV_R {
        ITV_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register is used to control wake-up and interrupt interval of RTC."]
    #[inline(always)]
    #[must_use]
    pub fn itv(&mut self) -> ITV_W<0> {
        ITV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC wake-up interval control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itv](index.html) module"]
pub struct ITV_SPEC;
impl crate::RegisterSpec for ITV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itv::R](R) reader structure"]
impl crate::Readable for ITV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itv::W](W) writer structure"]
impl crate::Writable for ITV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITV to value 0"]
impl crate::Resettable for ITV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

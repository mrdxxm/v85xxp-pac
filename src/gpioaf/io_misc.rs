#[doc = "Register `IO_MISC` reader"]
pub struct R(crate::R<IO_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_MISC` writer"]
pub struct W(crate::W<IO_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_MISC_SPEC>;
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
impl From<crate::W<IO_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLHDIV` reader - When IOB1 is selected to special function 3, this register is used to control the divide ratio of PLLH' s output."]
pub type PLLHDIV_R = crate::FieldReader<u8, PLLHDIV_A>;
#[doc = "When IOB1 is selected to special function 3, this register is used to control the divide ratio of PLLH' s output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLHDIV_A {
    #[doc = "0: `0`"]
    DIV1 = 0,
    #[doc = "1: `1`"]
    DIV2 = 1,
    #[doc = "2: `10`"]
    DIV4 = 2,
    #[doc = "3: `11`"]
    DIV8 = 3,
    #[doc = "4: `100`"]
    DIV16 = 4,
}
impl From<PLLHDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLHDIV_A) -> Self {
        variant as _
    }
}
impl PLLHDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLHDIV_A> {
        match self.bits {
            0 => Some(PLLHDIV_A::DIV1),
            1 => Some(PLLHDIV_A::DIV2),
            2 => Some(PLLHDIV_A::DIV4),
            3 => Some(PLLHDIV_A::DIV8),
            4 => Some(PLLHDIV_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLHDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLHDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLHDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLHDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLHDIV_A::DIV16
    }
}
#[doc = "Field `PLLHDIV` writer - When IOB1 is selected to special function 3, this register is used to control the divide ratio of PLLH' s output."]
pub type PLLHDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_MISC_SPEC, u8, PLLHDIV_A, 3, O>;
impl<'a, const O: u8> PLLHDIV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLHDIV_A::DIV1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLHDIV_A::DIV2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLHDIV_A::DIV4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLHDIV_A::DIV8)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLHDIV_A::DIV16)
    }
}
#[doc = "Field `I2CIOC` reader - This register is used to control the I2C function is at IOB or IOC."]
pub type I2CIOC_R = crate::BitReader<I2CIOC_A>;
#[doc = "This register is used to control the I2C function is at IOB or IOC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2CIOC_A {
    #[doc = "0: `0`"]
    IOB13_IOB14 = 0,
    #[doc = "1: `1`"]
    IOC4_IOC5 = 1,
}
impl From<I2CIOC_A> for bool {
    #[inline(always)]
    fn from(variant: I2CIOC_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CIOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CIOC_A {
        match self.bits {
            false => I2CIOC_A::IOB13_IOB14,
            true => I2CIOC_A::IOC4_IOC5,
        }
    }
    #[doc = "Checks if the value of the field is `IOB13_IOB14`"]
    #[inline(always)]
    pub fn is_iob13_iob14(&self) -> bool {
        *self == I2CIOC_A::IOB13_IOB14
    }
    #[doc = "Checks if the value of the field is `IOC4_IOC5`"]
    #[inline(always)]
    pub fn is_ioc4_ioc5(&self) -> bool {
        *self == I2CIOC_A::IOC4_IOC5
    }
}
#[doc = "Field `I2CIOC` writer - This register is used to control the I2C function is at IOB or IOC."]
pub type I2CIOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IO_MISC_SPEC, I2CIOC_A, O>;
impl<'a, const O: u8> I2CIOC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iob13_iob14(self) -> &'a mut W {
        self.variant(I2CIOC_A::IOB13_IOB14)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ioc4_ioc5(self) -> &'a mut W {
        self.variant(I2CIOC_A::IOC4_IOC5)
    }
}
impl R {
    #[doc = "Bits 0:2 - When IOB1 is selected to special function 3, this register is used to control the divide ratio of PLLH' s output."]
    #[inline(always)]
    pub fn pllhdiv(&self) -> PLLHDIV_R {
        PLLHDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - This register is used to control the I2C function is at IOB or IOC."]
    #[inline(always)]
    pub fn i2cioc(&self) -> I2CIOC_R {
        I2CIOC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - When IOB1 is selected to special function 3, this register is used to control the divide ratio of PLLH' s output."]
    #[inline(always)]
    #[must_use]
    pub fn pllhdiv(&mut self) -> PLLHDIV_W<0> {
        PLLHDIV_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to control the I2C function is at IOB or IOC."]
    #[inline(always)]
    #[must_use]
    pub fn i2cioc(&mut self) -> I2CIOC_W<5> {
        I2CIOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO misc. control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_misc](index.html) module"]
pub struct IO_MISC_SPEC;
impl crate::RegisterSpec for IO_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_misc::R](R) reader structure"]
impl crate::Readable for IO_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_misc::W](W) writer structure"]
impl crate::Writable for IO_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO_MISC to value 0"]
impl crate::Resettable for IO_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

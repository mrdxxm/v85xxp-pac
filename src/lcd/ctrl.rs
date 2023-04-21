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
#[doc = "Field `FRQ` reader - LCD scan frequency."]
pub type FRQ_R = crate::FieldReader<u8, FRQ_A>;
#[doc = "LCD scan frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQ_A {
    #[doc = "0: `0`"]
    _64HZ = 0,
    #[doc = "1: `1`"]
    _128HZ = 1,
    #[doc = "2: `10`"]
    _256HZ = 2,
    #[doc = "3: `11`"]
    _512HZ = 3,
}
impl From<FRQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQ_A) -> Self {
        variant as _
    }
}
impl FRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRQ_A {
        match self.bits {
            0 => FRQ_A::_64HZ,
            1 => FRQ_A::_128HZ,
            2 => FRQ_A::_256HZ,
            3 => FRQ_A::_512HZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64HZ`"]
    #[inline(always)]
    pub fn is_64hz(&self) -> bool {
        *self == FRQ_A::_64HZ
    }
    #[doc = "Checks if the value of the field is `_128HZ`"]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        *self == FRQ_A::_128HZ
    }
    #[doc = "Checks if the value of the field is `_256HZ`"]
    #[inline(always)]
    pub fn is_256hz(&self) -> bool {
        *self == FRQ_A::_256HZ
    }
    #[doc = "Checks if the value of the field is `_512HZ`"]
    #[inline(always)]
    pub fn is_512hz(&self) -> bool {
        *self == FRQ_A::_512HZ
    }
}
#[doc = "Field `FRQ` writer - LCD scan frequency."]
pub type FRQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, FRQ_A, 2, O>;
impl<'a, const O: u8> FRQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _64hz(self) -> &'a mut W {
        self.variant(FRQ_A::_64HZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut W {
        self.variant(FRQ_A::_128HZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _256hz(self) -> &'a mut W {
        self.variant(FRQ_A::_256HZ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _512hz(self) -> &'a mut W {
        self.variant(FRQ_A::_512HZ)
    }
}
#[doc = "Field `DRV` reader - LCD driving resister control register."]
pub type DRV_R = crate::FieldReader<u8, DRV_A>;
#[doc = "LCD driving resister control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRV_A {
    #[doc = "0: `0`"]
    _300KOHM = 0,
    #[doc = "1: `1`"]
    _600KOHM = 1,
    #[doc = "2: `10`"]
    _150KOHM = 2,
    #[doc = "3: `11`"]
    _200KOHM = 3,
}
impl From<DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: DRV_A) -> Self {
        variant as _
    }
}
impl DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRV_A {
        match self.bits {
            0 => DRV_A::_300KOHM,
            1 => DRV_A::_600KOHM,
            2 => DRV_A::_150KOHM,
            3 => DRV_A::_200KOHM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_300KOHM`"]
    #[inline(always)]
    pub fn is_300kohm(&self) -> bool {
        *self == DRV_A::_300KOHM
    }
    #[doc = "Checks if the value of the field is `_600KOHM`"]
    #[inline(always)]
    pub fn is_600kohm(&self) -> bool {
        *self == DRV_A::_600KOHM
    }
    #[doc = "Checks if the value of the field is `_150KOHM`"]
    #[inline(always)]
    pub fn is_150kohm(&self) -> bool {
        *self == DRV_A::_150KOHM
    }
    #[doc = "Checks if the value of the field is `_200KOHM`"]
    #[inline(always)]
    pub fn is_200kohm(&self) -> bool {
        *self == DRV_A::_200KOHM
    }
}
#[doc = "Field `DRV` writer - LCD driving resister control register."]
pub type DRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, DRV_A, 2, O>;
impl<'a, const O: u8> DRV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _300kohm(self) -> &'a mut W {
        self.variant(DRV_A::_300KOHM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _600kohm(self) -> &'a mut W {
        self.variant(DRV_A::_600KOHM)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _150kohm(self) -> &'a mut W {
        self.variant(DRV_A::_150KOHM)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _200kohm(self) -> &'a mut W {
        self.variant(DRV_A::_200KOHM)
    }
}
#[doc = "Field `TYPE` reader - LCD type control register."]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "LCD type control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: `0`"]
    _4_COM = 0,
    #[doc = "1: `1`"]
    _6_COM = 1,
    #[doc = "2: `10`"]
    _8_COM = 2,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            0 => TYPE_A::_4_COM,
            1 => TYPE_A::_6_COM,
            2 => TYPE_A::_8_COM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_COM`"]
    #[inline(always)]
    pub fn is_4_com(&self) -> bool {
        *self == TYPE_A::_4_COM
    }
    #[doc = "Checks if the value of the field is `_6_COM`"]
    #[inline(always)]
    pub fn is_6_com(&self) -> bool {
        *self == TYPE_A::_6_COM
    }
    #[doc = "Checks if the value of the field is `_8_COM`"]
    #[inline(always)]
    pub fn is_8_com(&self) -> bool {
        *self == TYPE_A::_8_COM
    }
}
#[doc = "Field `TYPE` writer - LCD type control register."]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, TYPE_A, 2, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _4_com(self) -> &'a mut W {
        self.variant(TYPE_A::_4_COM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _6_com(self) -> &'a mut W {
        self.variant(TYPE_A::_6_COM)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8_com(self) -> &'a mut W {
        self.variant(TYPE_A::_8_COM)
    }
}
#[doc = "Field `EN` reader - LCD controller enable register."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - LCD controller enable register."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - LCD scan frequency."]
    #[inline(always)]
    pub fn frq(&self) -> FRQ_R {
        FRQ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - LCD driving resister control register."]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - LCD type control register."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - LCD controller enable register."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD scan frequency."]
    #[inline(always)]
    #[must_use]
    pub fn frq(&mut self) -> FRQ_W<0> {
        FRQ_W::new(self)
    }
    #[doc = "Bits 2:3 - LCD driving resister control register."]
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<2> {
        DRV_W::new(self)
    }
    #[doc = "Bits 4:5 - LCD type control register."]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<4> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 7 - LCD controller enable register."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<7> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

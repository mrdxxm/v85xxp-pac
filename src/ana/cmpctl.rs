#[doc = "Register `CMPCTL` reader"]
pub struct R(crate::R<CMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCTL` writer"]
pub struct W(crate::W<CMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCTL_SPEC>;
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
impl From<crate::W<CMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1_CHK_FRQ` reader - CMP1 check data frequence."]
pub type CMP1_CHK_FRQ_R = crate::FieldReader<u8, CMP1_CHK_FRQ_A>;
#[doc = "CMP1 check data frequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP1_CHK_FRQ_A {
    #[doc = "0: `0`"]
    EVERY_CLOCK = 0,
    #[doc = "1: `1`"]
    EVERY_7_8125MS = 1,
    #[doc = "2: `10`"]
    EVERY_125MS = 2,
    #[doc = "3: `11`"]
    EVERY_250MS = 3,
    #[doc = "4: `100`"]
    EVERY_500MS = 4,
}
impl From<CMP1_CHK_FRQ_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP1_CHK_FRQ_A) -> Self {
        variant as _
    }
}
impl CMP1_CHK_FRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP1_CHK_FRQ_A> {
        match self.bits {
            0 => Some(CMP1_CHK_FRQ_A::EVERY_CLOCK),
            1 => Some(CMP1_CHK_FRQ_A::EVERY_7_8125MS),
            2 => Some(CMP1_CHK_FRQ_A::EVERY_125MS),
            3 => Some(CMP1_CHK_FRQ_A::EVERY_250MS),
            4 => Some(CMP1_CHK_FRQ_A::EVERY_500MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EVERY_CLOCK`"]
    #[inline(always)]
    pub fn is_every_clock(&self) -> bool {
        *self == CMP1_CHK_FRQ_A::EVERY_CLOCK
    }
    #[doc = "Checks if the value of the field is `EVERY_7_8125MS`"]
    #[inline(always)]
    pub fn is_every_7_8125ms(&self) -> bool {
        *self == CMP1_CHK_FRQ_A::EVERY_7_8125MS
    }
    #[doc = "Checks if the value of the field is `EVERY_125MS`"]
    #[inline(always)]
    pub fn is_every_125ms(&self) -> bool {
        *self == CMP1_CHK_FRQ_A::EVERY_125MS
    }
    #[doc = "Checks if the value of the field is `EVERY_250MS`"]
    #[inline(always)]
    pub fn is_every_250ms(&self) -> bool {
        *self == CMP1_CHK_FRQ_A::EVERY_250MS
    }
    #[doc = "Checks if the value of the field is `EVERY_500MS`"]
    #[inline(always)]
    pub fn is_every_500ms(&self) -> bool {
        *self == CMP1_CHK_FRQ_A::EVERY_500MS
    }
}
#[doc = "Field `CMP1_CHK_FRQ` writer - CMP1 check data frequence."]
pub type CMP1_CHK_FRQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMPCTL_SPEC, u8, CMP1_CHK_FRQ_A, 3, O>;
impl<'a, const O: u8> CMP1_CHK_FRQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn every_clock(self) -> &'a mut W {
        self.variant(CMP1_CHK_FRQ_A::EVERY_CLOCK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn every_7_8125ms(self) -> &'a mut W {
        self.variant(CMP1_CHK_FRQ_A::EVERY_7_8125MS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn every_125ms(self) -> &'a mut W {
        self.variant(CMP1_CHK_FRQ_A::EVERY_125MS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn every_250ms(self) -> &'a mut W {
        self.variant(CMP1_CHK_FRQ_A::EVERY_250MS)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn every_500ms(self) -> &'a mut W {
        self.variant(CMP1_CHK_FRQ_A::EVERY_500MS)
    }
}
#[doc = "Field `CMP1_THR_EN` reader - CMP1 data over this setting would trigger the interrupt enable"]
pub type CMP1_THR_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_THR_EN` writer - CMP1 data over this setting would trigger the interrupt enable"]
pub type CMP1_THR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
#[doc = "Field `CMP1_CHK_NUM` reader - CMP1 check data number of time. 0~15: 1 time ~ 16 times"]
pub type CMP1_CHK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP1_CHK_NUM` writer - CMP1 check data number of time. 0~15: 1 time ~ 16 times"]
pub type CMP1_CHK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMP2_CHK_FRQ` reader - CMP2 check data frequency."]
pub type CMP2_CHK_FRQ_R = crate::FieldReader<u8, CMP2_CHK_FRQ_A>;
#[doc = "CMP2 check data frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP2_CHK_FRQ_A {
    #[doc = "0: `0`"]
    EVERY_CLOCK = 0,
    #[doc = "1: `1`"]
    EVERY_7_8125MS = 1,
    #[doc = "2: `10`"]
    EVERY_125MS = 2,
    #[doc = "3: `11`"]
    EVERY_250MS = 3,
    #[doc = "4: `100`"]
    EVERY_500MS = 4,
}
impl From<CMP2_CHK_FRQ_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP2_CHK_FRQ_A) -> Self {
        variant as _
    }
}
impl CMP2_CHK_FRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP2_CHK_FRQ_A> {
        match self.bits {
            0 => Some(CMP2_CHK_FRQ_A::EVERY_CLOCK),
            1 => Some(CMP2_CHK_FRQ_A::EVERY_7_8125MS),
            2 => Some(CMP2_CHK_FRQ_A::EVERY_125MS),
            3 => Some(CMP2_CHK_FRQ_A::EVERY_250MS),
            4 => Some(CMP2_CHK_FRQ_A::EVERY_500MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EVERY_CLOCK`"]
    #[inline(always)]
    pub fn is_every_clock(&self) -> bool {
        *self == CMP2_CHK_FRQ_A::EVERY_CLOCK
    }
    #[doc = "Checks if the value of the field is `EVERY_7_8125MS`"]
    #[inline(always)]
    pub fn is_every_7_8125ms(&self) -> bool {
        *self == CMP2_CHK_FRQ_A::EVERY_7_8125MS
    }
    #[doc = "Checks if the value of the field is `EVERY_125MS`"]
    #[inline(always)]
    pub fn is_every_125ms(&self) -> bool {
        *self == CMP2_CHK_FRQ_A::EVERY_125MS
    }
    #[doc = "Checks if the value of the field is `EVERY_250MS`"]
    #[inline(always)]
    pub fn is_every_250ms(&self) -> bool {
        *self == CMP2_CHK_FRQ_A::EVERY_250MS
    }
    #[doc = "Checks if the value of the field is `EVERY_500MS`"]
    #[inline(always)]
    pub fn is_every_500ms(&self) -> bool {
        *self == CMP2_CHK_FRQ_A::EVERY_500MS
    }
}
#[doc = "Field `CMP2_CHK_FRQ` writer - CMP2 check data frequency."]
pub type CMP2_CHK_FRQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMPCTL_SPEC, u8, CMP2_CHK_FRQ_A, 3, O>;
impl<'a, const O: u8> CMP2_CHK_FRQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn every_clock(self) -> &'a mut W {
        self.variant(CMP2_CHK_FRQ_A::EVERY_CLOCK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn every_7_8125ms(self) -> &'a mut W {
        self.variant(CMP2_CHK_FRQ_A::EVERY_7_8125MS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn every_125ms(self) -> &'a mut W {
        self.variant(CMP2_CHK_FRQ_A::EVERY_125MS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn every_250ms(self) -> &'a mut W {
        self.variant(CMP2_CHK_FRQ_A::EVERY_250MS)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn every_500ms(self) -> &'a mut W {
        self.variant(CMP2_CHK_FRQ_A::EVERY_500MS)
    }
}
#[doc = "Field `CMP2_THR_EN` reader - CMP2 data over this setting would trigger the interrupt enable"]
pub type CMP2_THR_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP2_THR_EN` writer - CMP2 data over this setting would trigger the interrupt enable"]
pub type CMP2_THR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
#[doc = "Field `CMP2_CHK_NUM` reader - CMP2 check data number of time. 0~15: 1 time ~ 16 times"]
pub type CMP2_CHK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMP2_CHK_NUM` writer - CMP2 check data number of time. 0~15: 1 time ~ 16 times"]
pub type CMP2_CHK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMP1_INT_MASK_EN` reader - When CMP1_CNT exceeds the CMP1_THR and CMP1_THE_EN = 1, only one interrupt is generated."]
pub type CMP1_INT_MASK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_INT_MASK_EN` writer - When CMP1_CNT exceeds the CMP1_THR and CMP1_THE_EN = 1, only one interrupt is generated."]
pub type CMP1_INT_MASK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
#[doc = "Field `CMP1_IO_NODEB` reader - The CMP1 output of IO port de-bounce control."]
pub type CMP1_IO_NODEB_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_IO_NODEB` writer - The CMP1 output of IO port de-bounce control."]
pub type CMP1_IO_NODEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
#[doc = "Field `CMP2_INT_MASK_EN` reader - When CMP2_CNT exceeds the CMP2_THR and CMP2_THE_EN = 1, only one interrupt is generated."]
pub type CMP2_INT_MASK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP2_INT_MASK_EN` writer - When CMP2_CNT exceeds the CMP2_THR and CMP2_THE_EN = 1, only one interrupt is generated."]
pub type CMP2_INT_MASK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
#[doc = "Field `CMP2_IO_NODEB` reader - The CMP2 output of IO port de-bounce control."]
pub type CMP2_IO_NODEB_R = crate::BitReader<bool>;
#[doc = "Field `CMP2_IO_NODEB` writer - The CMP2 output of IO port de-bounce control."]
pub type CMP2_IO_NODEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
#[doc = "Field `VDDALARM_CHK_FRQ_SEL` reader - VDDALARM check data frequency"]
pub type VDDALARM_CHK_FRQ_SEL_R = crate::FieldReader<u8, VDDALARM_CHK_FRQ_SEL_A>;
#[doc = "VDDALARM check data frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDALARM_CHK_FRQ_SEL_A {
    #[doc = "0: `0`"]
    NO_CHECK = 0,
    #[doc = "1: `1`"]
    ALWAYS_CHECK_BY_32KHZ = 1,
    #[doc = "2: `10`"]
    CHECKED_EVERY_7_8125MS = 2,
}
impl From<VDDALARM_CHK_FRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDALARM_CHK_FRQ_SEL_A) -> Self {
        variant as _
    }
}
impl VDDALARM_CHK_FRQ_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDALARM_CHK_FRQ_SEL_A {
        match self.bits {
            0 => VDDALARM_CHK_FRQ_SEL_A::NO_CHECK,
            1 => VDDALARM_CHK_FRQ_SEL_A::ALWAYS_CHECK_BY_32KHZ,
            2 => VDDALARM_CHK_FRQ_SEL_A::CHECKED_EVERY_7_8125MS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline(always)]
    pub fn is_no_check(&self) -> bool {
        *self == VDDALARM_CHK_FRQ_SEL_A::NO_CHECK
    }
    #[doc = "Checks if the value of the field is `ALWAYS_CHECK_BY_32KHZ`"]
    #[inline(always)]
    pub fn is_always_check_by_32khz(&self) -> bool {
        *self == VDDALARM_CHK_FRQ_SEL_A::ALWAYS_CHECK_BY_32KHZ
    }
    #[doc = "Checks if the value of the field is `CHECKED_EVERY_7_8125MS`"]
    #[inline(always)]
    pub fn is_checked_every_7_8125ms(&self) -> bool {
        *self == VDDALARM_CHK_FRQ_SEL_A::CHECKED_EVERY_7_8125MS
    }
}
#[doc = "Field `VDDALARM_CHK_FRQ_SEL` writer - VDDALARM check data frequency"]
pub type VDDALARM_CHK_FRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMPCTL_SPEC, u8, VDDALARM_CHK_FRQ_SEL_A, 2, O>;
impl<'a, const O: u8> VDDALARM_CHK_FRQ_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_check(self) -> &'a mut W {
        self.variant(VDDALARM_CHK_FRQ_SEL_A::NO_CHECK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn always_check_by_32khz(self) -> &'a mut W {
        self.variant(VDDALARM_CHK_FRQ_SEL_A::ALWAYS_CHECK_BY_32KHZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn checked_every_7_8125ms(self) -> &'a mut W {
        self.variant(VDDALARM_CHK_FRQ_SEL_A::CHECKED_EVERY_7_8125MS)
    }
}
#[doc = "Field `PWR_DEB_SEL` reader - VDDALARM, VDCIN and AVCCDET de-bounce control register."]
pub type PWR_DEB_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_DEB_SEL` writer - VDDALARM, VDCIN and AVCCDET de-bounce control register."]
pub type PWR_DEB_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPCTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - CMP1 check data frequence."]
    #[inline(always)]
    pub fn cmp1_chk_frq(&self) -> CMP1_CHK_FRQ_R {
        CMP1_CHK_FRQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - CMP1 data over this setting would trigger the interrupt enable"]
    #[inline(always)]
    pub fn cmp1_thr_en(&self) -> CMP1_THR_EN_R {
        CMP1_THR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - CMP1 check data number of time. 0~15: 1 time ~ 16 times"]
    #[inline(always)]
    pub fn cmp1_chk_num(&self) -> CMP1_CHK_NUM_R {
        CMP1_CHK_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - CMP2 check data frequency."]
    #[inline(always)]
    pub fn cmp2_chk_frq(&self) -> CMP2_CHK_FRQ_R {
        CMP2_CHK_FRQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - CMP2 data over this setting would trigger the interrupt enable"]
    #[inline(always)]
    pub fn cmp2_thr_en(&self) -> CMP2_THR_EN_R {
        CMP2_THR_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - CMP2 check data number of time. 0~15: 1 time ~ 16 times"]
    #[inline(always)]
    pub fn cmp2_chk_num(&self) -> CMP2_CHK_NUM_R {
        CMP2_CHK_NUM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - When CMP1_CNT exceeds the CMP1_THR and CMP1_THE_EN = 1, only one interrupt is generated."]
    #[inline(always)]
    pub fn cmp1_int_mask_en(&self) -> CMP1_INT_MASK_EN_R {
        CMP1_INT_MASK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The CMP1 output of IO port de-bounce control."]
    #[inline(always)]
    pub fn cmp1_io_nodeb(&self) -> CMP1_IO_NODEB_R {
        CMP1_IO_NODEB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - When CMP2_CNT exceeds the CMP2_THR and CMP2_THE_EN = 1, only one interrupt is generated."]
    #[inline(always)]
    pub fn cmp2_int_mask_en(&self) -> CMP2_INT_MASK_EN_R {
        CMP2_INT_MASK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The CMP2 output of IO port de-bounce control."]
    #[inline(always)]
    pub fn cmp2_io_nodeb(&self) -> CMP2_IO_NODEB_R {
        CMP2_IO_NODEB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - VDDALARM check data frequency"]
    #[inline(always)]
    pub fn vddalarm_chk_frq_sel(&self) -> VDDALARM_CHK_FRQ_SEL_R {
        VDDALARM_CHK_FRQ_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - VDDALARM, VDCIN and AVCCDET de-bounce control register."]
    #[inline(always)]
    pub fn pwr_deb_sel(&self) -> PWR_DEB_SEL_R {
        PWR_DEB_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CMP1 check data frequence."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_chk_frq(&mut self) -> CMP1_CHK_FRQ_W<0> {
        CMP1_CHK_FRQ_W::new(self)
    }
    #[doc = "Bit 3 - CMP1 data over this setting would trigger the interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_thr_en(&mut self) -> CMP1_THR_EN_W<3> {
        CMP1_THR_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - CMP1 check data number of time. 0~15: 1 time ~ 16 times"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_chk_num(&mut self) -> CMP1_CHK_NUM_W<4> {
        CMP1_CHK_NUM_W::new(self)
    }
    #[doc = "Bits 8:10 - CMP2 check data frequency."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2_chk_frq(&mut self) -> CMP2_CHK_FRQ_W<8> {
        CMP2_CHK_FRQ_W::new(self)
    }
    #[doc = "Bit 11 - CMP2 data over this setting would trigger the interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2_thr_en(&mut self) -> CMP2_THR_EN_W<11> {
        CMP2_THR_EN_W::new(self)
    }
    #[doc = "Bits 12:15 - CMP2 check data number of time. 0~15: 1 time ~ 16 times"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2_chk_num(&mut self) -> CMP2_CHK_NUM_W<12> {
        CMP2_CHK_NUM_W::new(self)
    }
    #[doc = "Bit 16 - When CMP1_CNT exceeds the CMP1_THR and CMP1_THE_EN = 1, only one interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_int_mask_en(&mut self) -> CMP1_INT_MASK_EN_W<16> {
        CMP1_INT_MASK_EN_W::new(self)
    }
    #[doc = "Bit 17 - The CMP1 output of IO port de-bounce control."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_io_nodeb(&mut self) -> CMP1_IO_NODEB_W<17> {
        CMP1_IO_NODEB_W::new(self)
    }
    #[doc = "Bit 20 - When CMP2_CNT exceeds the CMP2_THR and CMP2_THE_EN = 1, only one interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2_int_mask_en(&mut self) -> CMP2_INT_MASK_EN_W<20> {
        CMP2_INT_MASK_EN_W::new(self)
    }
    #[doc = "Bit 21 - The CMP2 output of IO port de-bounce control."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2_io_nodeb(&mut self) -> CMP2_IO_NODEB_W<21> {
        CMP2_IO_NODEB_W::new(self)
    }
    #[doc = "Bits 22:23 - VDDALARM check data frequency"]
    #[inline(always)]
    #[must_use]
    pub fn vddalarm_chk_frq_sel(&mut self) -> VDDALARM_CHK_FRQ_SEL_W<22> {
        VDDALARM_CHK_FRQ_SEL_W::new(self)
    }
    #[doc = "Bits 24:31 - VDDALARM, VDCIN and AVCCDET de-bounce control register."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_deb_sel(&mut self) -> PWR_DEB_SEL_W<24> {
        PWR_DEB_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP1/CMP2 control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpctl](index.html) module"]
pub struct CMPCTL_SPEC;
impl crate::RegisterSpec for CMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpctl::R](R) reader structure"]
impl crate::Readable for CMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpctl::W](W) writer structure"]
impl crate::Writable for CMPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

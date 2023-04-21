#[doc = "Register `CMPOUT` reader"]
pub struct R(crate::R<CMPOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKH` reader - PLLH lock status."]
pub type LOCKH_R = crate::BitReader<bool>;
#[doc = "Field `LOCKL` reader - PLLL lock status."]
pub type LOCKL_R = crate::BitReader<bool>;
#[doc = "Field `CMP1` reader - This bit shows the output of comparator 1."]
pub type CMP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP2` reader - This bit shows the output of comparator 2."]
pub type CMP2_R = crate::BitReader<bool>;
#[doc = "Field `VDDALARM` reader - This bit shows the output of VDDALARM."]
pub type VDDALARM_R = crate::BitReader<bool>;
#[doc = "Field `VDCINDROP` reader - VDCIN drop status."]
pub type VDCINDROP_R = crate::BitReader<bool>;
#[doc = "Field `AVCCLV` reader - AVCCLV low power status."]
pub type AVCCLV_R = crate::BitReader<bool>;
#[doc = "Field `TADCO` reader - Tiny ADC output."]
pub type TADCO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - PLLH lock status."]
    #[inline(always)]
    pub fn lockh(&self) -> LOCKH_R {
        LOCKH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLLL lock status."]
    #[inline(always)]
    pub fn lockl(&self) -> LOCKL_R {
        LOCKL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit shows the output of comparator 1."]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit shows the output of comparator 2."]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit shows the output of VDDALARM."]
    #[inline(always)]
    pub fn vddalarm(&self) -> VDDALARM_R {
        VDDALARM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VDCIN drop status."]
    #[inline(always)]
    pub fn vdcindrop(&self) -> VDCINDROP_R {
        VDCINDROP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - AVCCLV low power status."]
    #[inline(always)]
    pub fn avcclv(&self) -> AVCCLV_R {
        AVCCLV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Tiny ADC output."]
    #[inline(always)]
    pub fn tadco(&self) -> TADCO_R {
        TADCO_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Comparator result register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpout](index.html) module"]
pub struct CMPOUT_SPEC;
impl crate::RegisterSpec for CMPOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpout::R](R) reader structure"]
impl crate::Readable for CMPOUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMPOUT to value 0x30"]
impl crate::Resettable for CMPOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}

#[doc = "Register `CH3TSR` reader"]
pub struct R(crate::R<CH3TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3TSR` writer"]
pub struct W(crate::W<CH3TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3TSR_SPEC>;
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
impl From<crate::W<CH3TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLKLEN` reader - BLKLEN"]
pub type BLKLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLKLEN` writer - BLKLEN"]
pub type BLKLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3TSR_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLKCNT` reader - BLKCNT"]
pub type BLKCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLKCNT` writer - BLKCNT"]
pub type BLKCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3TSR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - BLKLEN"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R {
        BLKLEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BLKLEN"]
    #[inline(always)]
    #[must_use]
    pub fn blklen(&mut self) -> BLKLEN_W<0> {
        BLKLEN_W::new(self)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BLKCNT_W<16> {
        BLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH3TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3tsr](index.html) module"]
pub struct CH3TSR_SPEC;
impl crate::RegisterSpec for CH3TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3tsr::R](R) reader structure"]
impl crate::Readable for CH3TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3tsr::W](W) writer structure"]
impl crate::Writable for CH3TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3TSR to value 0"]
impl crate::Resettable for CH3TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

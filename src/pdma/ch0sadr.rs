#[doc = "Register `CH0SADR` reader"]
pub struct R(crate::R<CH0SADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0SADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0SADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0SADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0SADR` writer"]
pub struct W(crate::W<CH0SADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0SADR_SPEC>;
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
impl From<crate::W<CH0SADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0SADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADR` reader - SADR"]
pub type SADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SADR` writer - SADR"]
pub type SADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0SADR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SADR_W<0> {
        SADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH0SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0sadr](index.html) module"]
pub struct CH0SADR_SPEC;
impl crate::RegisterSpec for CH0SADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0sadr::R](R) reader structure"]
impl crate::Readable for CH0SADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0sadr::W](W) writer structure"]
impl crate::Writable for CH0SADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0SADR to value 0"]
impl crate::Resettable for CH0SADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

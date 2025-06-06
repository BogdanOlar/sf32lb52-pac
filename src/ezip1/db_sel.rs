///Register `DB_SEL` reader
pub type R = crate::R<DB_SELrs>;
///Register `DB_SEL` writer
pub type W = crate::W<DB_SELrs>;
///Field `DB_SEL` reader - bit\[15\]
///0:line_first 1:out_buf_en\[1\]
///bit\[14\]
///0:rd_head3 1:out_buf_en\[0\]
///bit\[13\]
///0:rd_head2 1:inbuf_empty bit\[12\]
///0:rd_heas1 1:inbuf_half_empty bit\[11\]
///0:blk_restart 1: inbuf_full bit\[10\]
///0:ezip_buf_end 1:ezip_pixel_end bit\[9\]
///0:ezip_buf_full 1:0 bit\[8\]
///0:ezip_buf_empty 1:0 bit\[7\]
///0:dec_buf_empty 1:0 bit\[6\]
///0:dec_buf_full 1:para_ok bit\[5\]
///0:dec_on 1:ezip_fuf_push bit\[4\]
///0:ind3_on 1:copy_on bit\[3\]
///0:ind2_on 1:bypass_on bit\[2\]
///0:ind1_on 1:blk_clr bit\[1\]
///0:ezip_on 1:para_val bit\[0\]
///0:ezip_int 1:para_req
pub type DbSelR = crate::FieldReader<u16>;
///Field `DB_SEL` writer - bit\[15\]
///0:line_first 1:out_buf_en\[1\]
///bit\[14\]
///0:rd_head3 1:out_buf_en\[0\]
///bit\[13\]
///0:rd_head2 1:inbuf_empty bit\[12\]
///0:rd_heas1 1:inbuf_half_empty bit\[11\]
///0:blk_restart 1: inbuf_full bit\[10\]
///0:ezip_buf_end 1:ezip_pixel_end bit\[9\]
///0:ezip_buf_full 1:0 bit\[8\]
///0:ezip_buf_empty 1:0 bit\[7\]
///0:dec_buf_empty 1:0 bit\[6\]
///0:dec_buf_full 1:para_ok bit\[5\]
///0:dec_on 1:ezip_fuf_push bit\[4\]
///0:ind3_on 1:copy_on bit\[3\]
///0:ind2_on 1:bypass_on bit\[2\]
///0:ind1_on 1:blk_clr bit\[1\]
///0:ezip_on 1:para_val bit\[0\]
///0:ezip_int 1:para_req
pub type DbSelW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - bit\[15\]
    ///0:line_first 1:out_buf_en\[1\]
    ///bit\[14\]
    ///0:rd_head3 1:out_buf_en\[0\]
    ///bit\[13\]
    ///0:rd_head2 1:inbuf_empty bit\[12\]
    ///0:rd_heas1 1:inbuf_half_empty bit\[11\]
    ///0:blk_restart 1: inbuf_full bit\[10\]
    ///0:ezip_buf_end 1:ezip_pixel_end bit\[9\]
    ///0:ezip_buf_full 1:0 bit\[8\]
    ///0:ezip_buf_empty 1:0 bit\[7\]
    ///0:dec_buf_empty 1:0 bit\[6\]
    ///0:dec_buf_full 1:para_ok bit\[5\]
    ///0:dec_on 1:ezip_fuf_push bit\[4\]
    ///0:ind3_on 1:copy_on bit\[3\]
    ///0:ind2_on 1:bypass_on bit\[2\]
    ///0:ind1_on 1:blk_clr bit\[1\]
    ///0:ezip_on 1:para_val bit\[0\]
    ///0:ezip_int 1:para_req
    #[inline(always)]
    pub fn db_sel(&self) -> DbSelR {
        DbSelR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_SEL")
            .field("rsvd", &self.rsvd())
            .field("db_sel", &self.db_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - bit\[15\]
    ///0:line_first 1:out_buf_en\[1\]
    ///bit\[14\]
    ///0:rd_head3 1:out_buf_en\[0\]
    ///bit\[13\]
    ///0:rd_head2 1:inbuf_empty bit\[12\]
    ///0:rd_heas1 1:inbuf_half_empty bit\[11\]
    ///0:blk_restart 1: inbuf_full bit\[10\]
    ///0:ezip_buf_end 1:ezip_pixel_end bit\[9\]
    ///0:ezip_buf_full 1:0 bit\[8\]
    ///0:ezip_buf_empty 1:0 bit\[7\]
    ///0:dec_buf_empty 1:0 bit\[6\]
    ///0:dec_buf_full 1:para_ok bit\[5\]
    ///0:dec_on 1:ezip_fuf_push bit\[4\]
    ///0:ind3_on 1:copy_on bit\[3\]
    ///0:ind2_on 1:bypass_on bit\[2\]
    ///0:ind1_on 1:blk_clr bit\[1\]
    ///0:ezip_on 1:para_val bit\[0\]
    ///0:ezip_int 1:para_req
    #[inline(always)]
    pub fn db_sel(&mut self) -> DbSelW<DB_SELrs> {
        DbSelW::new(self, 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DB_SELrs> {
        RsvdW::new(self, 16)
    }
}
///ezip decoder debug sel
///
///You can [`read`](crate::Reg::read) this register and get [`db_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_SELrs;
impl crate::RegisterSpec for DB_SELrs {
    type Ux = u32;
}
///`read()` method returns [`db_sel::R`](R) reader structure
impl crate::Readable for DB_SELrs {}
///`write(|w| ..)` method takes [`db_sel::W`](W) writer structure
impl crate::Writable for DB_SELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_SEL to value 0
impl crate::Resettable for DB_SELrs {
    const RESET_VALUE: u32 = 0;
}

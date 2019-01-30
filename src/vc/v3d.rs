#![allow(dead_code)]

pub mod command_builder;
pub mod defs;

use self::defs::*;
use crate::vc::memory::{map_physical_memory, Allocation};
use std::error::Error;

macro_rules! impl_reg {
    ($get:ident, $set:ident, $offset:expr) => {
        pub fn $get(&self) -> u32 {
            unsafe {
                (self.allocation.address as *const u32)
                    .offset($offset as isize / 4)
                    .read_volatile()
            }
        }

        pub fn $set(&mut self, value: u32) {
            unsafe {
                (self.allocation.address as *mut u32)
                    .offset($offset as isize / 4)
                    .write_volatile(value);
            }
        }
    };
}

pub struct V3d {
    allocation: Allocation,
}

impl V3d {
    pub fn new() -> Result<V3d, Box<dyn Error>> {
        Ok(V3d {
            allocation: map_physical_memory(BASE, 0x1000)?,
        })
    }

    impl_reg!(ident0, set_ident0, IDENT0);
    impl_reg!(ident1, set_ident1, IDENT1);
    impl_reg!(ident2, set_ident2, IDENT2);
    impl_reg!(ident3, set_ident3, IDENT3);
    impl_reg!(scratch, set_scratch, SCRATCH);
    impl_reg!(l2cactl, set_l2cactl, L2CACTL);
    impl_reg!(slcactl, set_slcactl, SLCACTL);
    impl_reg!(intctl, set_intctl, INTCTL);
    impl_reg!(intena, set_intena, INTENA);
    impl_reg!(intdis, set_intdis, INTDIS);
    impl_reg!(ct0cs, set_ct0cs, CT0CS);
    impl_reg!(ct1cs, set_ct1cs, CT1CS);
    impl_reg!(ct0ea, set_ct0ea, CT0EA);
    impl_reg!(ct1ea, set_ct1ea, CT1EA);
    impl_reg!(ct0ca, set_ct0ca, CT0CA);
    impl_reg!(ct1ca, set_ct1ca, CT1CA);
    impl_reg!(ct0ra0, set_ct0ra0, CT0RA0);
    impl_reg!(ct1ra0, set_ct1ra0, CT1RA0);
    impl_reg!(ct0lc, set_ct0lc, CT0LC);
    impl_reg!(ct1lc, set_ct1lc, CT1LC);
    impl_reg!(ct0pc, set_ct0pc, CT0PC);
    impl_reg!(ct1pc, set_ct1pc, CT1PC);
    impl_reg!(pcs, set_pcs, PCS);
    impl_reg!(bfc, set_bfc, BFC);
    impl_reg!(rfc, set_rfc, RFC);
    impl_reg!(bpca, set_bpca, BPCA);
    impl_reg!(bpcs, set_bpcs, BPCS);
    impl_reg!(bpoa, set_bpoa, BPOA);
    impl_reg!(bpos, set_bpos, BPOS);
    impl_reg!(bxcf, set_bxcf, BXCF);
    impl_reg!(sqrsv0, set_sqrsv0, SQRSV0);
    impl_reg!(sqrsv1, set_sqrsv1, SQRSV1);
    impl_reg!(sqcntl, set_sqcntl, SQCNTL);
    impl_reg!(sqcstat, set_sqcstat, SQCSTAT);
    impl_reg!(srqpc, set_srqpc, SRQPC);
    impl_reg!(srqua, set_srqua, SRQUA);
    impl_reg!(srqul, set_srqul, SRQUL);
    impl_reg!(srqcs, set_srqcs, SRQCS);
    impl_reg!(vpacntl, set_vpacntl, VPACNTL);
    impl_reg!(vpmbase, set_vpmbase, VPMBASE);
    impl_reg!(pctrc, set_pctrc, PCTRC);
    impl_reg!(pctre, set_pctre, PCTRE);
    impl_reg!(pctr0, set_pctr0, PCTR0);
    impl_reg!(pctrs0, set_pctrs0, PCTRS0);
    impl_reg!(pctr1, set_pctr1, PCTR1);
    impl_reg!(pctrs1, set_pctrs1, PCTRS1);
    impl_reg!(pctr2, set_pctr2, PCTR2);
    impl_reg!(pctrs2, set_pctrs2, PCTRS2);
    impl_reg!(pctr3, set_pctr3, PCTR3);
    impl_reg!(pctrs3, set_pctrs3, PCTRS3);
    impl_reg!(pctr4, set_pctr4, PCTR4);
    impl_reg!(pctrs4, set_pctrs4, PCTRS4);
    impl_reg!(pctr5, set_pctr5, PCTR5);
    impl_reg!(pctrs5, set_pctrs5, PCTRS5);
    impl_reg!(pctr6, set_pctr6, PCTR6);
    impl_reg!(pctrs6, set_pctrs6, PCTRS6);
    impl_reg!(pctr7, set_pctr7, PCTR7);
    impl_reg!(pctrs7, set_pctrs7, PCTRS7);
    impl_reg!(pctr8, set_pctr8, PCTR8);
    impl_reg!(pctrs8, set_pctrs8, PCTRS8);
    impl_reg!(pctr9, set_pctr9, PCTR9);
    impl_reg!(pctrs9, set_pctrs9, PCTRS9);
    impl_reg!(pctr10, set_pctr10, PCTR10);
    impl_reg!(pctrs10, set_pctrs10, PCTRS10);
    impl_reg!(pctr11, set_pctr11, PCTR11);
    impl_reg!(pctrs11, set_pctrs11, PCTRS11);
    impl_reg!(pctr12, set_pctr12, PCTR12);
    impl_reg!(pctrs12, set_pctrs12, PCTRS12);
    impl_reg!(pctr13, set_pctr13, PCTR13);
    impl_reg!(pctrs13, set_pctrs13, PCTRS13);
    impl_reg!(pctr14, set_pctr14, PCTR14);
    impl_reg!(pctrs14, set_pctrs14, PCTRS14);
    impl_reg!(pctr15, set_pctr15, PCTR15);
    impl_reg!(pctrs15, set_pctrs15, PCTRS15);
    impl_reg!(dbcfg, set_dbcfg, DBCFG);
    impl_reg!(dbscs, set_dbscs, DBSCS);
    impl_reg!(dbscfg, set_dbscfg, DBSCFG);
    impl_reg!(dbssr, set_dbssr, DBSSR);
    impl_reg!(dbsdr0, set_dbsdr0, DBSDR0);
    impl_reg!(dbsdr1, set_dbsdr1, DBSDR1);
    impl_reg!(dbsdr2, set_dbsdr2, DBSDR2);
    impl_reg!(dbsdr3, set_dbsdr3, DBSDR3);
    impl_reg!(dbqrun, set_dbqrun, DBQRUN);
    impl_reg!(dbqhlt, set_dbqhlt, DBQHLT);
    impl_reg!(dbqstp, set_dbqstp, DBQSTP);
    impl_reg!(dbqite, set_dbqite, DBQITE);
    impl_reg!(dbqitc, set_dbqitc, DBQITC);
    impl_reg!(dbqghc, set_dbqghc, DBQGHC);
    impl_reg!(dbqghg, set_dbqghg, DBQGHG);
    impl_reg!(dbqghh, set_dbqghh, DBQGHH);
    impl_reg!(dbge, set_dbge, DBGE);
    impl_reg!(fdbgo, set_fdbgo, FDBGO);
    impl_reg!(fdbgb, set_fdbgb, FDBGB);
    impl_reg!(fdbgr, set_fdbgr, FDBGR);
    impl_reg!(fdbgs, set_fdbgs, FDBGS);
    impl_reg!(errstat, set_errstat, ERRSTAT);
}

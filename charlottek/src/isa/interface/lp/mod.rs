//! # Logical Processor Control Interface

pub trait LpControlIfce {
    type LicId;
    type LpId;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn curr_lic_id() -> Self::LicId;
    fn curr_lp_id() -> Self::LpId;
}

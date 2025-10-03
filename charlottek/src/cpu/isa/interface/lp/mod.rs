//! # Logical Processor Control Interface

pub trait LpIfce {
    type HwAsid;
    type LicId;
    type LpId;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn read_lic_id() -> Self::LicId;
    fn write_lp_id(lp_id: Self::LpId);
    fn read_lp_id() -> Self::LpId;
}

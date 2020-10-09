pub struct HidrawDevice {
    pub vendor_id: u16,
    pub product_id: u16,
}

impl std::fmt::Debug for HidrawDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(VendorId = {:04x}, ProductId = {:04x})",
            self.vendor_id, self.product_id
        )
    }
}

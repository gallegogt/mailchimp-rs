
// ============ E-Commerce stats for a campaign. ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ECommerceReportType {
    /// The total orders for a campaign.
    #[serde(default)]
    pub total_orders: u64,
    /// The total spent for a campaign. Calculated as the sum of
    /// all order totals with no deductions.
    #[serde(default)]
    pub total_spent: f32,
    /// The total revenue for a campaign. Calculated as the sum of
    /// all order totals minus shipping and tax totals.
    #[serde(default)]
    pub total_revenue: f32,
}

impl Default for ECommerceReportType {
    fn default() -> Self {
        ECommerceReportType {
            total_orders: 0,
            total_spent: 0.0,
            total_revenue: 0.0,
        }
    }
}
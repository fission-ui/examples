pub struct StripeService;

impl StripeService {
    pub async fn create_payment_intent(_amount: f64) -> anyhow::Result<String> {
        Ok("intent_placeholder".to_string())
    }
}

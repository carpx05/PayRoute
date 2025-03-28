use crate::models::{PaymentRequest, RoutingResponse};
use crate::config::Rule;

pub fn route_payment(
    request: &PaymentRequest,
    rules: &[Rule],
) -> Option<PaymentResponse> {
    for rule in rules {
        if request.amount >= rule.min_amount 
            && request.amount <= rule.max_amount 
            && request.currency == rule.currency {
            return Some(RoutingResponse {
                selected_psp: rule.psp.to_string(),
            });
        }
    }
    None
}
use bigdecimal::BigDecimal;
use crate::response_status::ResponseStatus;
use crate::TradingResponse;

pub struct OrderResponse {
    pub id: String,
    pub status: ResponseStatus,
    pub cum_qty: Option<i32>,
    pub price_unit: Option<BigDecimal>,
}

impl TradingResponse for OrderResponse {
    fn receive_response(&self) {
        match self.status {
            ResponseStatus::NEW => { println!("Received NEW for {}", self.id) }
            ResponseStatus::CANCEL => { println!("Received CANCEL for {}", self.id) }
            ResponseStatus::FILL => {
                println!("RECEIVED FILL for {}", self.id);
                match self.cum_qty {
                    None => {}
                    Some(qtd) => {
                        println!("Executed {} for {}", qtd, self.price_unit.clone().expect("Not Found Price Unit"))
                    }
                }
            }
            ResponseStatus::EXPIRED => { println!("RECEIVED EXPIRED for {}", self.id) }
            ResponseStatus::REJECT => { println!("RECEIVED REJECT for {}", self.id) }
        }
    }
}

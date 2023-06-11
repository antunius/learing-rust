use crate::side::Side;
use crate::status_order::StatusOrder;
use crate::Trading;

pub struct Order {
    pub id: String,
    pub quantity: i32,
    pub symbol: String,
    pub side: Side,
    pub status: StatusOrder,
}

impl Trading for Order {
    fn receive(&self) {
        println!("Order {}", self.id);
        match self.side {
            Side::BUY => {
                println!("Buying {} {}", self.quantity, self.symbol)
            }
            Side::SELL => {
                println!("Selling {} {}", self.quantity, self.symbol)
            }
        }
    }

    fn cancel(&mut self) {
        self.status = StatusOrder::CANCEL
    }
}

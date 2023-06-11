use std::str::FromStr;

use bigdecimal::BigDecimal;
use ulid::Ulid;
use order::Order;
use order_response::OrderResponse;
use response_status::ResponseStatus;

use side::Side;
use status_order::StatusOrder;

mod side;
mod status_order;
mod response_status;
mod order;
mod order_response;

trait Trading {
    fn receive(&self);
    fn cancel(&mut self);
}

trait TradingResponse {
    fn receive_response(&self);
}


fn main() {
    let order_buy = Box::new(Order {
        id: Ulid::new().to_string(),
        quantity: 10,
        symbol: "GOOGL".to_string(),
        side: Side::BUY,
        status: StatusOrder::PENDING,
    });
    order_buy.receive();

    let order_response = OrderResponse {
        id: order_buy.id.clone(),
        status: ResponseStatus::NEW,
        cum_qty: None,
        price_unit: None,
    };

    order_response.receive_response();

    let order_response = OrderResponse {
        id: order_buy.id.clone(),
        status: ResponseStatus::FILL,
        cum_qty: Some(order_buy.quantity),
        price_unit: Some(BigDecimal::from_str("122.23").unwrap()),
    };

    order_response.receive_response();

    let order_sell = Box::new(Order {
        id: Ulid::new().to_string(),
        quantity: 10,
        symbol: "GOOGL".to_string(),
        side: Side::SELL,
        status: StatusOrder::PENDING,
    });

    order_sell.receive()
}

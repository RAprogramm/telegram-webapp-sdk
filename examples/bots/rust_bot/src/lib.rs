// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Order data structure received from the Burger King demo WebApp
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderData {
    /// Unique order identifier
    pub id:          u32,
    /// Item name
    pub name:        String,
    /// Price in cents
    pub price_cents: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_data_deserialize() {
        let json = r#"{"id": 1, "name": "Whopper", "price_cents": 599}"#;
        let order: OrderData = serde_json::from_str(json).expect("valid json");

        assert_eq!(order.id, 1);
        assert_eq!(order.name, "Whopper");
        assert_eq!(order.price_cents, 599);
    }

    #[test]
    fn test_order_data_serialize() {
        let order = OrderData { id: 2, name: String::from("Big King"), price_cents: 499 };

        let json = serde_json::to_string(&order).expect("serialize");

        assert!(json.contains("\"id\":2"));
        assert!(json.contains("\"name\":\"Big King\""));
        assert!(json.contains("\"price_cents\":499"));
    }

    #[test]
    fn test_order_data_missing_field() {
        let json = r#"{"id": 1, "name": "Whopper"}"#;
        let result: Result<OrderData, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_order_price_calculation() {
        let order = OrderData { id: 1, name: String::from("Test"), price_cents: 1234 };

        let price_dollars = order.price_cents as f64 / 100.0;

        assert_eq!(price_dollars, 12.34);
    }

    #[test]
    fn test_order_data_roundtrip() {
        let original = OrderData { id: 42, name: String::from("Chicken Royale"), price_cents: 750 };

        let json = serde_json::to_string(&original).expect("serialize");
        let deserialized: OrderData = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(original, deserialized);
    }
}

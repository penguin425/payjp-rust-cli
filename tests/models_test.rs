use serde_json::json;

#[test]
fn test_charge_deserialize() {
    let json_data = json!({
        "id": "ch_test_123",
        "object": "charge",
        "livemode": false,
        "created": 1704067200,
        "amount": 3500,
        "currency": "jpy",
        "paid": true,
        "captured": true,
        "refunded": false,
        "amount_refunded": 0,
        "card": {
            "id": "car_test_456",
            "object": "card",
            "created": 1704067200,
            "livemode": false,
            "name": null,
            "last4": "4242",
            "exp_month": 12,
            "exp_year": 2025,
            "brand": "Visa",
            "fingerprint": "fp_xxx"
        },
        "metadata": {
            "order_id": "12345"
        }
    });

    let charge: payjp_cli::models::Charge = serde_json::from_value(json_data).unwrap();
    assert_eq!(charge.id, "ch_test_123");
    assert_eq!(charge.amount, 3500);
    assert_eq!(charge.currency, "jpy");
    assert!(charge.paid);
    assert!(charge.captured);
    assert!(!charge.refunded);
    assert!(charge.card.is_some());

    let card = charge.card.unwrap();
    assert_eq!(card.last4, "4242");
    assert_eq!(card.brand, "Visa");

    assert_eq!(charge.metadata.get("order_id"), Some(&"12345".to_string()));
}

#[test]
fn test_customer_deserialize() {
    let json_data = json!({
        "id": "cus_test_789",
        "object": "customer",
        "livemode": false,
        "created": 1704067200,
        "default_card": "car_test_456",
        "email": "test@example.com",
        "description": "Test customer",
        "cards": {
            "object": "list",
            "count": 1,
            "has_more": false,
            "url": "/v1/customers/cus_test_789/cards",
            "data": []
        },
        "metadata": {}
    });

    let customer: payjp_cli::models::Customer = serde_json::from_value(json_data).unwrap();
    assert_eq!(customer.id, "cus_test_789");
    assert_eq!(customer.email, Some("test@example.com".to_string()));
    assert_eq!(customer.default_card, Some("car_test_456".to_string()));
}

#[test]
fn test_card_deserialize() {
    let json_data = json!({
        "id": "car_test_456",
        "object": "card",
        "created": 1704067200,
        "livemode": false,
        "name": "TARO YAMADA",
        "last4": "4242",
        "exp_month": 12,
        "exp_year": 2025,
        "brand": "Visa",
        "fingerprint": "fp_xxx",
        "country": "JP",
        "metadata": {}
    });

    let card: payjp_cli::models::Card = serde_json::from_value(json_data).unwrap();
    assert_eq!(card.id, "car_test_456");
    assert_eq!(card.last4, "4242");
    assert_eq!(card.exp_month, 12);
    assert_eq!(card.exp_year, 2025);
    assert_eq!(card.brand, "Visa");
    assert_eq!(card.name, Some("TARO YAMADA".to_string()));
}

#[test]
fn test_list_deserialize() {
    let json_data = json!({
        "object": "list",
        "count": 2,
        "has_more": true,
        "url": "/v1/charges",
        "data": [
            {
                "id": "ch_test_1",
                "object": "charge",
                "livemode": false,
                "created": 1704067200,
                "amount": 1000,
                "currency": "jpy",
                "paid": true,
                "captured": true,
                "refunded": false,
                "amount_refunded": 0,
                "metadata": {}
            },
            {
                "id": "ch_test_2",
                "object": "charge",
                "livemode": false,
                "created": 1704067300,
                "amount": 2000,
                "currency": "jpy",
                "paid": true,
                "captured": true,
                "refunded": false,
                "amount_refunded": 0,
                "metadata": {}
            }
        ]
    });

    let list: payjp_cli::models::List<payjp_cli::models::Charge> = serde_json::from_value(json_data).unwrap();
    assert_eq!(list.count, 2);
    assert!(list.has_more);
    assert_eq!(list.data.len(), 2);
    assert_eq!(list.data[0].id, "ch_test_1");
    assert_eq!(list.data[1].id, "ch_test_2");
}

#[test]
fn test_error_deserialize() {
    let json_data = json!({
        "error": {
            "status": 402,
            "type": "card_error",
            "code": "card_declined",
            "message": "Card was declined",
            "param": "card"
        }
    });

    let error: payjp_cli::models::PayjpApiError = serde_json::from_value(json_data).unwrap();
    assert_eq!(error.error.status, 402);
    assert_eq!(error.error.error_type, "card_error");
    assert_eq!(error.error.code, Some("card_declined".to_string()));
    assert_eq!(error.error.message, "Card was declined");
    assert_eq!(error.error.param, Some("card".to_string()));
}

#[test]
fn test_error_hint() {
    let detail = payjp_cli::models::PayjpErrorDetail {
        status: 402,
        error_type: "card_error".to_string(),
        code: Some("card_declined".to_string()),
        message: "Card was declined".to_string(),
        param: Some("card".to_string()),
    };

    let hint = detail.get_hint();
    assert!(hint.is_some());
    assert!(hint.unwrap().contains("カード会社"));
}

#[test]
fn test_delete_response_deserialize() {
    let json_data = json!({
        "id": "cus_test_789",
        "object": "customer",
        "deleted": true,
        "livemode": false
    });

    let response: payjp_cli::models::DeleteResponse = serde_json::from_value(json_data).unwrap();
    assert_eq!(response.id, "cus_test_789");
    assert!(response.deleted);
}

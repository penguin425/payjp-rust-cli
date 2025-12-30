# PAY.JP CLI テスト仕様書

## 概要

本ドキュメントでは、PAY.JP CLI のテスト内容について説明します。

## テスト実行方法

```bash
# 全テスト実行
cargo test

# 特定のテストを実行
cargo test test_charge_deserialize

# 詳細出力付きで実行
cargo test -- --nocapture
```

## テスト一覧

| テスト名 | カテゴリ | 説明 |
|---------|---------|------|
| `test_charge_deserialize` | モデル | Charge の JSON デシリアライズ |
| `test_customer_deserialize` | モデル | Customer の JSON デシリアライズ |
| `test_card_deserialize` | モデル | Card の JSON デシリアライズ |
| `test_list_deserialize` | モデル | List レスポンスのデシリアライズ |
| `test_error_deserialize` | モデル | API エラーのデシリアライズ |
| `test_error_hint` | エラー処理 | 日本語ヒントメッセージ生成 |
| `test_delete_response_deserialize` | モデル | 削除レスポンスのデシリアライズ |

## テスト詳細

### 1. test_charge_deserialize

**目的**: PAY.JP API の Charge レスポンスが正しくパースできることを確認

**テストデータ**:
```json
{
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
}
```

**検証項目**:
- `id` が `"ch_test_123"` であること
- `amount` が `3500` であること
- `currency` が `"jpy"` であること
- `paid` が `true` であること
- `captured` が `true` であること
- `refunded` が `false` であること
- `card` がネストしたオブジェクトとして存在すること
- `card.last4` が `"4242"` であること
- `card.brand` が `"Visa"` であること
- `metadata["order_id"]` が `"12345"` であること

---

### 2. test_customer_deserialize

**目的**: Customer レスポンスが正しくパースできることを確認

**テストデータ**:
```json
{
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
}
```

**検証項目**:
- `id` が `"cus_test_789"` であること
- `email` が `Some("test@example.com")` であること
- `default_card` が `Some("car_test_456")` であること

---

### 3. test_card_deserialize

**目的**: Card オブジェクトが正しくパースできることを確認

**テストデータ**:
```json
{
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
}
```

**検証項目**:
- `id` が `"car_test_456"` であること
- `last4` が `"4242"` であること
- `exp_month` が `12` であること
- `exp_year` が `2025` であること
- `brand` が `"Visa"` であること
- `name` が `Some("TARO YAMADA")` であること

---

### 4. test_list_deserialize

**目的**: ページネーション付きリストレスポンスが正しくパースできることを確認

**テストデータ**:
```json
{
  "object": "list",
  "count": 2,
  "has_more": true,
  "url": "/v1/charges",
  "data": [
    {
      "id": "ch_test_1",
      "object": "charge",
      "amount": 1000,
      ...
    },
    {
      "id": "ch_test_2",
      "object": "charge",
      "amount": 2000,
      ...
    }
  ]
}
```

**検証項目**:
- `count` が `2` であること
- `has_more` が `true` であること
- `data` 配列の長さが `2` であること
- `data[0].id` が `"ch_test_1"` であること
- `data[1].id` が `"ch_test_2"` であること

---

### 5. test_error_deserialize

**目的**: API エラーレスポンスが正しくパースできることを確認

**テストデータ**:
```json
{
  "error": {
    "status": 402,
    "type": "card_error",
    "code": "card_declined",
    "message": "Card was declined",
    "param": "card"
  }
}
```

**検証項目**:
- `error.status` が `402` であること
- `error.error_type` が `"card_error"` であること
- `error.code` が `Some("card_declined")` であること
- `error.message` が `"Card was declined"` であること
- `error.param` が `Some("card")` であること

---

### 6. test_error_hint

**目的**: エラーコードに応じた日本語ヒントメッセージが正しく生成されることを確認

**テストデータ**:
```rust
PayjpErrorDetail {
    status: 402,
    error_type: "card_error",
    code: Some("card_declined"),
    message: "Card was declined",
    param: Some("card"),
}
```

**検証項目**:
- `get_hint()` が `Some` を返すこと
- ヒントメッセージに「カード会社」が含まれること

**エラーコードとヒントの対応表**:

| エラーコード | ヒントメッセージ |
|-------------|----------------|
| `invalid_number` | カード番号が正しいかご確認ください。 |
| `invalid_cvc` | セキュリティコード（CVC）が正しいかご確認ください。 |
| `invalid_expiry_month` | 有効期限の月（1-12）が正しいかご確認ください。 |
| `invalid_expiry_year` | 有効期限の年が正しいかご確認ください。 |
| `expired_card` | カードの有効期限が切れています。別のカードをお試しください。 |
| `card_declined` | カード会社にお問い合わせいただくか、別のカードをお試しください。 |
| `processing_error` | 一時的なエラーです。しばらく待ってから再度お試しください。 |
| `invalid_api_key` | APIキーが正しいかご確認ください。 |
| `over_capacity` | レート制限に達しました。しばらく待ってから再度お試しください。 |
| `three_d_secure_failed` | 3Dセキュア認証に失敗しました。再度お試しください。 |

---

### 7. test_delete_response_deserialize

**目的**: 削除 API のレスポンスが正しくパースできることを確認

**テストデータ**:
```json
{
  "id": "cus_test_789",
  "object": "customer",
  "deleted": true,
  "livemode": false
}
```

**検証項目**:
- `id` が `"cus_test_789"` であること
- `deleted` が `true` であること

---

## 今後追加予定のテスト

### 統合テスト（API 呼び出し）

```rust
#[test]
#[ignore] // 実際の API を呼び出すため通常は無視
fn test_create_token_and_charge() {
    // 1. トークン作成
    // 2. トークンで決済
    // 3. 決済情報の確認
}
```

### CLI テスト

```rust
#[test]
fn test_cli_charge_list() {
    let output = Command::new("payjp")
        .args(["charge", "list", "--limit", "5"])
        .env("PAYJP_SECRET_KEY", "sk_test_xxx")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}
```

### 設定テスト

```rust
#[test]
fn test_config_priority() {
    // CLI > ENV > File の優先順位を確認
}

#[test]
fn test_config_file_read_write() {
    // 設定ファイルの読み書きを確認
}
```

---

## テストカバレッジ

現在のテストは以下をカバーしています：

| カテゴリ | カバレッジ |
|---------|----------|
| モデルのデシリアライズ | ✅ 完了 |
| エラーヒント生成 | ✅ 完了 |
| API クライアント | ❌ 未実装 |
| CLI コマンド | ❌ 未実装 |
| 設定管理 | ❌ 未実装 |
| 出力フォーマット | ❌ 未実装 |

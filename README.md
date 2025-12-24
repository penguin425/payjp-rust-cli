# PAY.JP CLI

PAY.JP 決済サービスの API を操作する CLI ツールです。Rust で実装されています。

## インストール

### ソースからビルド

```bash
git clone https://github.com/penguin425/payjp-rust-cli.git
cd payjp-rust-cli
cargo build --release
```

ビルドされたバイナリは `target/release/payjp` にあります。

### Cargo でインストール

```bash
cargo install --path .
```

## 設定

### API キーの設定

PAY.JP では2種類のキーを使用します：
- **秘密鍵（sk_xxx）**: サーバーサイド操作（Charge, Customer, Card）
- **公開鍵（pk_xxx）**: トークン作成

以下の方法で API キーを設定できます（優先順位順）：

1. **コマンドラインオプション**
   ```bash
   # 秘密鍵
   payjp -k sk_test_xxx charge list

   # 公開鍵（トークン作成用）
   payjp --public-key pk_test_xxx token create ...
   ```

2. **環境変数**
   ```bash
   export PAYJP_SECRET_KEY=sk_test_xxx
   export PAYJP_PUBLIC_KEY=pk_test_xxx
   ```

3. **設定ファイル**
   ```bash
   payjp config set --api-key sk_test_xxx --public-key pk_test_xxx
   ```

### 設定ファイルの場所

- Linux/macOS: `~/.config/payjp/config.toml`
- Windows: `%APPDATA%\payjp\config.toml`

### プロファイル

複数の環境を管理できます：

```bash
# テスト環境（デフォルト）
payjp config set --api-key sk_test_xxx --public-key pk_test_xxx

# 本番環境
payjp config set --profile production --api-key sk_live_xxx --public-key pk_live_xxx

# 本番環境を使用
payjp -p production charge list
```

## 使い方

### 基本的な使い方

```bash
payjp [OPTIONS] <COMMAND>

Options:
  -k, --api-key <KEY>       秘密鍵（sk_xxx）
  --public-key <KEY>        公開鍵（pk_xxx）
  -o, --output <FORMAT>     出力形式 [json|table]
  -v, --verbose             詳細出力
  -p, --profile <NAME>      使用するプロファイル
  -h, --help                ヘルプ表示
  -V, --version             バージョン表示

Commands:
  token     トークン操作（公開鍵使用）
  charge    支払い操作
  customer  顧客操作
  card      カード操作
  config    設定管理
```

### Token（トークン）操作

トークン作成は公開鍵を使用します：

```bash
# トークン作成（テストカード使用）
payjp --public-key pk_test_xxx token create \
  --number 4242424242424242 \
  --exp-month 12 \
  --exp-year 2025 \
  --cvc 123

# トークン作成（カード名義付き）
payjp token create \
  --number 4242424242424242 \
  --exp-month 12 \
  --exp-year 2025 \
  --cvc 123 \
  --name "TARO YAMADA"

# トークン取得
payjp token get tok_xxx
```

### Charge（支払い）操作

```bash
# 支払い作成（カードトークン使用）
payjp charge create --amount 3500 --card tok_xxx

# 支払い作成（顧客ID使用）
payjp charge create --amount 5000 --customer cus_xxx

# 支払い作成（オーソリのみ、後で確定）
payjp charge create --amount 3500 --card tok_xxx --capture=false

# 支払い取得
payjp charge get ch_xxx

# 支払い更新
payjp charge update ch_xxx --description "注文 #12345"

# 返金（全額）
payjp charge refund ch_xxx

# 返金（一部）
payjp charge refund ch_xxx --amount 1000

# 支払い確定
payjp charge capture ch_xxx

# 支払い一覧
payjp charge list --limit 20

# 日付範囲で絞り込み
payjp charge list --since 2024-01-01 --until 2024-12-31
```

### Customer（顧客）操作

```bash
# 顧客作成
payjp customer create --email user@example.com

# 顧客作成（カード付き）
payjp customer create --email user@example.com --card tok_xxx

# 顧客取得
payjp customer get cus_xxx

# 顧客更新
payjp customer update cus_xxx --email new@example.com

# 顧客削除
payjp customer delete cus_xxx

# 顧客一覧
payjp customer list --limit 10
```

### Card（カード）操作

```bash
# カード作成
payjp card create cus_xxx --card tok_xxx

# カード取得
payjp card get cus_xxx car_xxx

# カード更新
payjp card update cus_xxx car_xxx --name "TARO YAMADA"

# カード削除
payjp card delete cus_xxx car_xxx

# カード一覧
payjp card list cus_xxx
```

### 出力形式

```bash
# テーブル形式（デフォルト）
payjp charge list

# JSON形式
payjp -o json charge list

# JSON形式で設定
payjp config set --output json
```

## 典型的なワークフロー

### 1. トークン作成から決済まで

```bash
# 1. 公開鍵でトークン作成
payjp --public-key pk_test_xxx token create \
  --number 4242424242424242 \
  --exp-month 12 \
  --exp-year 2025 \
  --cvc 123

# 2. 秘密鍵で決済
payjp -k sk_test_xxx charge create --amount 3500 --card tok_xxx
```

### 2. 顧客登録から定期課金まで

```bash
# 1. トークン作成
payjp token create --number 4242424242424242 --exp-month 12 --exp-year 2025 --cvc 123

# 2. 顧客作成（トークンを紐付け）
payjp customer create --email user@example.com --card tok_xxx

# 3. 顧客に課金
payjp charge create --amount 1000 --customer cus_xxx
```

## メタデータ

支払いや顧客にメタデータを追加できます：

```bash
# メタデータ付きで支払い作成
payjp charge create --amount 3500 --card tok_xxx \
  --metadata order_id=12345 \
  --metadata user_id=u_xxx

# メタデータ付きで顧客作成
payjp customer create --email user@example.com \
  --metadata plan=premium \
  --metadata source=website
```

## 3Dセキュア

```bash
# 3Dセキュア有効で支払い作成
payjp charge create --amount 3500 --card tok_xxx --three-d-secure

# 3Dセキュア完了
payjp charge tds-finish ch_xxx
```

## エラーハンドリング

CLI はエラーを日本語で分かりやすく表示します：

```
Error: カードエラー

  Code:    card_declined
  Message: カード会社に拒否されました
  Param:   card

Hint: カード会社にお問い合わせいただくか、別のカードをお試しください。
```

## テスト用カード番号

| カード番号 | 結果 |
|-----------|------|
| 4242424242424242 | 成功 |
| 4000000000000002 | カード拒否 |
| 4000000000000069 | 有効期限切れ |
| 4000000000000119 | 処理エラー |

## 開発

### ビルド

```bash
cargo build
```

### テスト

```bash
cargo test
```

### リリースビルド

```bash
cargo build --release
```

## ライセンス

MIT License

## 関連リンク

- [PAY.JP 公式ドキュメント](https://pay.jp/docs/api/)
- [PAY.JP ダッシュボード](https://pay.jp/)

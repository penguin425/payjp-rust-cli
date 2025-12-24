use crate::config::OutputFormat;
use crate::models::{Card, Charge, Customer, DeleteResponse, List, Token};
use chrono::{FixedOffset, TimeZone};
use colored::Colorize;
use serde::Serialize;
use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table, Tabled,
};

/// Format Unix timestamp to JST string
pub fn format_timestamp(ts: i64) -> String {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    match jst.timestamp_opt(ts, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S JST").to_string(),
        _ => "Invalid timestamp".to_string(),
    }
}

/// Format amount with currency
pub fn format_amount(amount: u64, currency: &str) -> String {
    match currency.to_lowercase().as_str() {
        "jpy" => format!("¥{}", amount),
        _ => format!("{} {}", amount, currency.to_uppercase()),
    }
}

/// Output helper
pub struct Output {
    format: OutputFormat,
}

impl Output {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Output JSON or formatted data
    pub fn print<T: Serialize + TableDisplay>(&self, data: &T) {
        match self.format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(data).unwrap());
            }
            OutputFormat::Table => {
                data.print_table();
            }
        }
    }

    /// Output a list
    pub fn print_list<T: Serialize + TableDisplay>(&self, list: &List<T>) {
        match self.format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(list).unwrap());
            }
            OutputFormat::Table => {
                if list.data.is_empty() {
                    println!("{}", "No items found.".dimmed());
                } else {
                    T::print_list(&list.data);
                    println!();
                    println!(
                        "{}",
                        format!(
                            "Showing {} of {} items{}",
                            list.data.len(),
                            list.count,
                            if list.has_more { " (more available)" } else { "" }
                        )
                        .dimmed()
                    );
                }
            }
        }
    }

    /// Output delete response
    pub fn print_delete(&self, response: &DeleteResponse) {
        match self.format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(response).unwrap());
            }
            OutputFormat::Table => {
                println!(
                    "{} {} (ID: {})",
                    "Deleted".green().bold(),
                    response.object,
                    response.id
                );
            }
        }
    }
}

/// Trait for table display
pub trait TableDisplay: Sized {
    fn print_table(&self);
    fn print_list(items: &[Self]);
}

/// Table row for Charge
#[derive(Tabled)]
struct ChargeRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Paid")]
    paid: String,
    #[tabled(rename = "Captured")]
    captured: String,
    #[tabled(rename = "Refunded")]
    refunded: String,
    #[tabled(rename = "Card")]
    card: String,
    #[tabled(rename = "Created")]
    created: String,
}

impl From<&Charge> for ChargeRow {
    fn from(charge: &Charge) -> Self {
        let card_info = charge
            .card
            .as_ref()
            .map(|c| format!("{} ****{}", c.brand, c.last4))
            .unwrap_or_else(|| "-".to_string());

        Self {
            id: charge.id.clone(),
            amount: format_amount(charge.amount, &charge.currency),
            paid: if charge.paid { "✓".green().to_string() } else { "✗".red().to_string() },
            captured: if charge.captured { "✓".green().to_string() } else { "✗".yellow().to_string() },
            refunded: if charge.refunded {
                format!("✓ ({})", format_amount(charge.amount_refunded, &charge.currency))
            } else {
                "✗".to_string()
            },
            card: card_info,
            created: format_timestamp(charge.created),
        }
    }
}

impl TableDisplay for Charge {
    fn print_table(&self) {
        println!("{}", "Charge Details".bold());
        println!("{}", "─".repeat(50));
        println!("  {:15} {}", "ID:".dimmed(), self.id);
        println!("  {:15} {}", "Amount:".dimmed(), format_amount(self.amount, &self.currency));
        println!("  {:15} {}", "Paid:".dimmed(), if self.paid { "Yes".green() } else { "No".red() });
        println!("  {:15} {}", "Captured:".dimmed(), if self.captured { "Yes".green() } else { "No".yellow() });
        if let Some(captured_at) = self.captured_at {
            println!("  {:15} {}", "Captured At:".dimmed(), format_timestamp(captured_at));
        }
        println!("  {:15} {}", "Refunded:".dimmed(), if self.refunded { "Yes".red() } else { "No".green() });
        if self.amount_refunded > 0 {
            println!("  {:15} {}", "Refund Amount:".dimmed(), format_amount(self.amount_refunded, &self.currency));
        }
        if let Some(ref reason) = self.refund_reason {
            println!("  {:15} {}", "Refund Reason:".dimmed(), reason);
        }
        if let Some(ref card) = self.card {
            println!("  {:15} {} ****{} ({}/{})", "Card:".dimmed(), card.brand, card.last4, card.exp_month, card.exp_year);
        }
        if let Some(ref customer) = self.customer {
            println!("  {:15} {}", "Customer:".dimmed(), customer);
        }
        if let Some(ref desc) = self.description {
            println!("  {:15} {}", "Description:".dimmed(), desc);
        }
        if let Some(ref status) = self.three_d_secure_status {
            println!("  {:15} {}", "3D Secure:".dimmed(), status);
        }
        if let Some(ref code) = self.failure_code {
            println!("  {:15} {}", "Failure Code:".dimmed(), code.red());
        }
        if let Some(ref msg) = self.failure_message {
            println!("  {:15} {}", "Failure Msg:".dimmed(), msg.red());
        }
        println!("  {:15} {}", "Live Mode:".dimmed(), if self.livemode { "Yes" } else { "No (Test)" });
        println!("  {:15} {}", "Created:".dimmed(), format_timestamp(self.created));
        if !self.metadata.is_empty() {
            println!("  {:15}", "Metadata:".dimmed());
            for (k, v) in &self.metadata {
                println!("    {} = {}", k, v);
            }
        }
    }

    fn print_list(items: &[Self]) {
        let rows: Vec<ChargeRow> = items.iter().map(ChargeRow::from).collect();
        let table = Table::new(rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::first()).with(Width::truncate(24).suffix("...")))
            .to_string();
        println!("{}", table);
    }
}

/// Table row for Customer
#[derive(Tabled)]
struct CustomerRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Email")]
    email: String,
    #[tabled(rename = "Cards")]
    cards: String,
    #[tabled(rename = "Default Card")]
    default_card: String,
    #[tabled(rename = "Created")]
    created: String,
}

impl From<&Customer> for CustomerRow {
    fn from(customer: &Customer) -> Self {
        Self {
            id: customer.id.clone(),
            email: customer.email.clone().unwrap_or_else(|| "-".to_string()),
            cards: customer.cards.count.to_string(),
            default_card: customer.default_card.clone().unwrap_or_else(|| "-".to_string()),
            created: format_timestamp(customer.created),
        }
    }
}

impl TableDisplay for Customer {
    fn print_table(&self) {
        println!("{}", "Customer Details".bold());
        println!("{}", "─".repeat(50));
        println!("  {:15} {}", "ID:".dimmed(), self.id);
        if let Some(ref email) = self.email {
            println!("  {:15} {}", "Email:".dimmed(), email);
        }
        if let Some(ref desc) = self.description {
            println!("  {:15} {}", "Description:".dimmed(), desc);
        }
        if let Some(ref card) = self.default_card {
            println!("  {:15} {}", "Default Card:".dimmed(), card);
        }
        println!("  {:15} {}", "Cards:".dimmed(), self.cards.count);
        println!("  {:15} {}", "Live Mode:".dimmed(), if self.livemode { "Yes" } else { "No (Test)" });
        println!("  {:15} {}", "Created:".dimmed(), format_timestamp(self.created));
        if !self.metadata.is_empty() {
            println!("  {:15}", "Metadata:".dimmed());
            for (k, v) in &self.metadata {
                println!("    {} = {}", k, v);
            }
        }

        // Print cards if any
        if !self.cards.data.is_empty() {
            println!();
            println!("  {}", "Cards:".bold());
            for card in &self.cards.data {
                let default_marker = if Some(&card.id) == self.default_card.as_ref() {
                    " (default)".green()
                } else {
                    "".normal()
                };
                println!(
                    "    - {} {} ****{} ({}/{}){}",
                    card.id.dimmed(),
                    card.brand,
                    card.last4,
                    card.exp_month,
                    card.exp_year,
                    default_marker
                );
            }
        }
    }

    fn print_list(items: &[Self]) {
        let rows: Vec<CustomerRow> = items.iter().map(CustomerRow::from).collect();
        let table = Table::new(rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::first()).with(Width::truncate(24).suffix("...")))
            .to_string();
        println!("{}", table);
    }
}

/// Table row for Card
#[derive(Tabled)]
struct CardRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Brand")]
    brand: String,
    #[tabled(rename = "Last4")]
    last4: String,
    #[tabled(rename = "Exp")]
    exp: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Created")]
    created: String,
}

impl From<&Card> for CardRow {
    fn from(card: &Card) -> Self {
        Self {
            id: card.id.clone(),
            brand: card.brand.clone(),
            last4: card.last4.clone(),
            exp: format!("{:02}/{}", card.exp_month, card.exp_year),
            name: card.name.clone().unwrap_or_else(|| "-".to_string()),
            created: format_timestamp(card.created),
        }
    }
}

impl TableDisplay for Card {
    fn print_table(&self) {
        println!("{}", "Card Details".bold());
        println!("{}", "─".repeat(50));
        println!("  {:15} {}", "ID:".dimmed(), self.id);
        println!("  {:15} {}", "Brand:".dimmed(), self.brand);
        println!("  {:15} ****{}", "Number:".dimmed(), self.last4);
        println!("  {:15} {:02}/{}", "Expiry:".dimmed(), self.exp_month, self.exp_year);
        if let Some(ref name) = self.name {
            println!("  {:15} {}", "Name:".dimmed(), name);
        }
        println!("  {:15} {}", "Fingerprint:".dimmed(), self.fingerprint);
        if let Some(ref country) = self.country {
            println!("  {:15} {}", "Country:".dimmed(), country);
        }
        if let Some(ref cvc) = self.cvc_check {
            println!("  {:15} {}", "CVC Check:".dimmed(), cvc);
        }
        if let Some(ref zip) = self.address_zip_check {
            println!("  {:15} {}", "ZIP Check:".dimmed(), zip);
        }
        if let Some(ref status) = self.three_d_secure_status {
            println!("  {:15} {}", "3D Secure:".dimmed(), status);
        }
        println!("  {:15} {}", "Live Mode:".dimmed(), if self.livemode { "Yes" } else { "No (Test)" });
        println!("  {:15} {}", "Created:".dimmed(), format_timestamp(self.created));

        // Address
        let has_address = self.address_line1.is_some()
            || self.address_line2.is_some()
            || self.address_city.is_some()
            || self.address_state.is_some()
            || self.address_zip.is_some();
        if has_address {
            println!("  {:15}", "Address:".dimmed());
            if let Some(ref line1) = self.address_line1 {
                println!("    {}", line1);
            }
            if let Some(ref line2) = self.address_line2 {
                println!("    {}", line2);
            }
            let city_state = [
                self.address_city.clone(),
                self.address_state.clone(),
                self.address_zip.clone(),
            ]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
            if !city_state.is_empty() {
                println!("    {}", city_state);
            }
        }

        if !self.metadata.is_empty() {
            println!("  {:15}", "Metadata:".dimmed());
            for (k, v) in &self.metadata {
                println!("    {} = {}", k, v);
            }
        }
    }

    fn print_list(items: &[Self]) {
        let rows: Vec<CardRow> = items.iter().map(CardRow::from).collect();
        let table = Table::new(rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::first()).with(Width::truncate(24).suffix("...")))
            .to_string();
        println!("{}", table);
    }
}

/// Table row for Token
#[derive(Tabled)]
struct TokenRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Card")]
    card: String,
    #[tabled(rename = "Used")]
    used: String,
    #[tabled(rename = "Created")]
    created: String,
}

impl From<&Token> for TokenRow {
    fn from(token: &Token) -> Self {
        Self {
            id: token.id.clone(),
            card: format!("{} ****{}", token.card.brand, token.card.last4),
            used: if token.used { "✓".yellow().to_string() } else { "✗".green().to_string() },
            created: format_timestamp(token.created),
        }
    }
}

impl TableDisplay for Token {
    fn print_table(&self) {
        println!("{}", "Token Details".bold());
        println!("{}", "─".repeat(50));
        println!("  {:15} {}", "ID:".dimmed(), self.id.green());
        println!("  {:15} {}", "Used:".dimmed(), if self.used { "Yes".yellow() } else { "No".green() });
        println!("  {:15} {}", "Live Mode:".dimmed(), if self.livemode { "Yes" } else { "No (Test)" });
        println!("  {:15} {}", "Created:".dimmed(), format_timestamp(self.created));
        println!();
        println!("  {}", "Card Information:".bold());
        println!("    {:13} {}", "Brand:".dimmed(), self.card.brand);
        println!("    {:13} ****{}", "Number:".dimmed(), self.card.last4);
        println!("    {:13} {:02}/{}", "Expiry:".dimmed(), self.card.exp_month, self.card.exp_year);
        if let Some(ref name) = self.card.name {
            println!("    {:13} {}", "Name:".dimmed(), name);
        }
        if let Some(ref cvc) = self.card.cvc_check {
            println!("    {:13} {}", "CVC Check:".dimmed(), cvc);
        }
    }

    fn print_list(items: &[Self]) {
        let rows: Vec<TokenRow> = items.iter().map(TokenRow::from).collect();
        let table = Table::new(rows)
            .with(Style::rounded())
            .with(Modify::new(Columns::first()).with(Width::truncate(28).suffix("...")))
            .to_string();
        println!("{}", table);
    }
}

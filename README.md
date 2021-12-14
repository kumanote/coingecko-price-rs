# coingecko-price-rs

> coingecko-price-rs is a very simple library to fetch price information from Coingecko with its api.

## Installation

#### Dependencies

- [Rust with Cargo](http://rust-lang.org)

**rust-toolchain**

```text
1.54.0
```

#### Importing

**Cargo.toml**

```toml
[dependencies]
coingecko-price = { version = "0.1.0", git = "https://github.com/kumanote/coingecko-price-rs", branch = "main" }
```

**rust files**

```rust
use coingecko_price;
```

## Usage

```rust
use coingecko_price::SimplePriceRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = SimplePriceRequest {
        ids: vec!["bitcoin,ethereum,cosmos".into()],
        vs_currencies: vec!["usd".into()],
        include_market_cap: None,
        include_24hr_vol: None,
        include_24hr_change: None,
        include_last_updated_at: None,
    };
    let response = coingecko_price::simple_price(request).await?;
    println!("{:?}", response);
    // {"bitcoin": {"usd": 47495}, "cosmos": {"usd": 21.8}, "ethereum": {"usd": 3823.14}}
    Ok(())
}
```

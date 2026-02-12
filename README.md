# Sentiment Commet (Dene)

_A decentralized sentiment analysis and feedback system for Maison Dorée, powered by Stellar Soroban._

## Overview

**Sentiment Commet** (Project Codename: Dene) is a full-stack Rust web application that collects customer reviews, performs basic sentiment analysis, and **independently verifies negative feedback** by storing a random sample of negative comments on the Stellar blockchain using a custom Soroban smart contract.

This "reservoir sampling" mechanism ensures that negative feedback cannot be censored or ignored by the centralized database administrators, providing transparency and trust for both the establishment and its customers.

## Features

-   **Full-Stack Rust**: Built with [Leptos](https://github.com/leptos-rs/leptos) for both SSR (Server-Side Rendering) and hydration.
-   **MongoDB Storage**: Efficiently stores all comments and metadata.
-   **Soroban Smart Contract**: Implements Algorithm R (Reservoir Sampling) to maintain a fixed-size, statistically representative sample of negative comments on-chain.
-   **Freighter Wallet Integration**: Users sign transactions directly from their browser (Client-Side Signing) to submit negative feedback transparently.
-   **Dynamic SDK Injection**: Robust handling of Stellar SDK loading to prevent browser compatibility issues.

## Tech Stack

-   **Frontend/Backend**: Rust, Leptos 0.7 (SSR + Hydration), Actix-Web
-   **Database**: MongoDB
-   **Blockchain**: Stellar Soroban (Testnet)
-   **Smart Contract**: Rust (no_std)
-   **Wallet**: Freighter (Browser Extension)
-   **Tools**: `cargo-leptos`, `soroban-cli`

## Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable)
-   `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
-   [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos`
-   [MongoDB](https://www.mongodb.com/try/download/community) (running locally or Atlas URI)
-   [Freighter Wallet](https://www.freighter.app/) extension installed in your browser.

## Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/mustafater/Sentiment_commet.git
    cd Sentiment_commet
    ```

2.  **Environment Setup:**
    Copy the example environment file and configure your MongoDB URI and Soroban settings.
    ```bash
    cp .env.example .env
    ```
    *Note: The project comes with a pre-configured Testnet contract ID in `.env`.*

3.  **Run the Application:**
    ```bash
    cargo leptos watch
    ```
    The app will be available at `http://localhost:3000`.

## Smart Contract

The core innovation is the `negative_sampler` contract (`contracts/negative_sampler`).

-   **Logic**: It maintains a maximum of `k` negative comments (default: 10).
-   **Sampling**: When a new negative comment is submitted:
    -   If the reservoir is not full (< `k`), it is added.
    -   If full, it replaces an existing comment with probability `k/n` (where `n` is the total number of negative comments seen).
-   **Transparency**: Anyone can verify the total count of negative comments seen vs. the stored sample.

### Deploying the Contract (Optional)

If you want to deploy your own instance:

```bash
# 1. Build
soroban contract build

# 2. Deploy
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/negative_sampler.wasm \
    --source <YOUR_IDENTITY> \
    --network testnet
```

## Usage

1.  Open the app at `http://localhost:3000`.
2.  Navigate to the **Community** page.
3.  Connect your Freighter wallet.
4.  Post a review:
    -   **Positive/Neutral**: Saved to MongoDB only.
    -   **Negative**: Prompted to sign a transaction with Freighter to submit to the Soroban contract.

## Testing

You can verify the contract state using the included CLI script:

```bash
./test_contract_cli.sh
```

## License

MIT

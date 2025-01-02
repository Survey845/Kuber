# KUBER - A secure token wallet on the ICP blockchain

Kuber is a wallet implementation written in Rust and deployed on the ICP blockchain. It supports basic functionalities for token exchange.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Setup](#setup)
- [Testing](#testing)
- [Documentation](#documentation)

## Overview

Kuber is made of two major components

- **Backend**: Smart Contracts written in Rust, using the Canister Development Kit provided by Dfinity.
- **Frontend**: React framework which lets the user access smart contract functions.

## Features

- **Send Tokens**: This function allows the wallet owner to send tokens to other wallets.
- **Receive Tokens**: This functions allows the wallet owner to add tokens to their wallets.
- **Get Balance**: This functions shows the balance in the owner's wallet.

## Setup

### Prerequisites

- Rust
- dfx
- NPM and Node.js

### Build

1. **Clone the repository**

```bash
 git clone https://github.com/survey845/Kuber.git
 cd my_wallet
```

2. **Start and Deploy**
   Start the dfx server on your local machine

```bash
dfx start --background
```

Deploy the code

```bash
dfx deploy
```

You may be prompted to enter your password, according to the identity used for you dfx configuration

3. **Open Frontend**  
   Once deployed, your application will be accessible at `http://localhost:4943?canisterId={your_canister_id}`

## Testing

Run tests

```bash
cargo test
```

## Documentation

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

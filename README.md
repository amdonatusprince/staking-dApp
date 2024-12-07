# Concordium Staking Protocol

A CIS-20 supported staking protocol built on the Concordium blockchain enables users to stake EUROe tokens and earn daily rewards. The protocol provides a secure, transparent, and efficient way to participate in staking while maintaining full control over assets.

## Overview

The Concordium Staking Protocol allows users to:
- Stake EUROe tokens and earn daily rewards
- Unstake tokens with a 1-day unbonding period
- Claim rewards at any time
- Track staking positions and rewards in real-time

### Key Features

- **Secure Staking**: Built on Concordium with the CIS-20 smart contract standards
- **Daily Rewards**: Earn rewards calculated based on stake amount and duration
- **Flexible Unstaking**: Withdraw stakes after a 1-day unbonding period
- **Transparent Operations**: All actions are recorded on-chain with detailed events
- **User-Friendly Interface**: Easy-to-use web interface for all staking operations

## Technical Architecture

### Smart Contract

The protocol consists of a main staking contract written in Rust for the Concordium blockchain. Key components include:

- **State Management**: Tracks all staking positions, rewards, and protocol parameters
- **Security Features**: Includes pause mechanism, admin controls, and signature verification
- **Event System**: Comprehensive event logging for all protocol actions

### Frontend Application

Built with React and Next.js, providing:
- Wallet integration with Concordium Browser Wallet
- Real-time staking information
- User-friendly staking and unstaking interface
- Detailed staking position tracking

## Contract Functions

### Core Functions

1. **Staking**
   - Stake EUROe tokens
   - Automatically starts earning rewards
   - Updates total staked amount and participant count

2. **Unstaking**
   - Initiate unstaking with 1-day unbonding period
   - Complete unstaking after unbonding period
   - Automatic reward calculation and distribution

3. **Rewards**
   - Daily reward accrual based on APR
   - Claim rewards at any time
   - Transparent reward calculation

### Administrative Functions

- Pause/unpause contract
- Update APR
- Emergency functions for security
- Contract upgrades

## Getting Started

### Prerequisites

- Concordium Browser Wallet
- EUROe tokens
- Modern web browser

### Installation

1. Clone the repository
```bash
git clone https://github.com/your-username/concordium-staking.git
cd concordium-staking
```
2. Install dependencies
```bash
// Install contract dependencies
cd contract
cargo build

// Install frontend dependencies
cd frontend
npm install
```
3.  Run development server
```bash
npm run dev
```

## Useful Resources

- **Demo Website**: https://concordium-staking-dapp.vercel.app/ 
- **Technical Guide**: Detailed documentation covering contract architecture, functions, and implementation details can be found [here](https://donatusprince.medium.com/a-developers-guide-to-building-a-fullstack-staking-dapp-on-the-concordium-blockchain-e5b67d5530ea)
- **Concordium Guide**: Step-by-step instructions for installing the concordium client in the [documentation](https://developer.concordium.software/en/mainnet/smart-contracts/guides/setup-tools.html)
- **Frontend Interface**: [Simple UI for interacting with the staking contract](https://concordium-staking-dapp.vercel.app/)
- **Concordium Documentation:** https://developer.concordium.software/
- **Concordium Website:** https://www.concordium.com/
- **Concordium Support:** https://support.concordium.software/
- **Discord:** https://discord.com/invite/GpKGE2hCFx
- 

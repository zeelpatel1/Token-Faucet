# 🪙 Solana Token Faucet (Anchor)

A fully working **Token Faucet Program** built using **Anchor Framework** on Solana.  
This faucet allows users to **request SPL tokens** from a **vault PDA**, ensuring secure and controlled token distribution.

This project includes:

✅ Anchor Program  
✅ PDA-based vault  
✅ Secure drip mechanism  
✅ Full test suite (minting, faucet init, drip)  
✅ Localnet setup  
✅ TypeScript client tests  

---

## 📌 Features

### ✔ Initialize Faucet  
Creates a PDA-owned vault containing the faucet’s supply.

### ✔ Drip Tokens  
Users can request a fixed amount of tokens — authority is always the PDA, not the user.

### ✔ Secure Architecture  
- PDA owns the vault token account  
- User only signs *their own* accounts  
- Token movement is always performed by PDA signer seeds  

---

## 🚀 Getting Started

```bash
npm install
solana-test-validator
anchor build
anchor deploy

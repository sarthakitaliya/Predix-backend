# Predix - Prediction Market (Off-Chain Matching + Solana On-Chain Settlement)

Predix is a high-performance prediction market backend built in Rust.  
It performs order matching off-chain, sends settlement instructions to Solana, and synchronizes on-chain state back into the database through an event listener.

The Anchor program and frontend UI are maintained in separate repositories.

## Related Repositories

- **Frontend (Client UI):** https://github.com/sarthakitaliya/predix-frontend  
- **On-Chain Anchor Program:** https://github.com/sarthakitaliya/solana-prediction-market-program  


## Architecture

<img width="1033" height="751" alt="Screenshot 2025-12-12 at 6 10 13 AM" src="https://github.com/user-attachments/assets/787ee239-753c-4f90-82f0-a44ee414799e" />

## Features

- Off-chain Matching Engine for instant order matching  
- Tokio task per market for isolated concurrent execution  
- `mpsc` + `oneshot` channels for efficient internal communication  
- Privy authentication and wallet integration  
- On-chain settlement via external Anchor program  
- Event Listener to persist on-chain state changes into Postgres  
- Admin panel for market creation, resolution, and management  

## Flow (high level)

1. **Client authenticates using Privy**  
   User sessions and signatures are handled through Privy.

2. **Client submits an order**  
   Backend validates the request and forwards the order to the Matching Engine.

3. **Matching Engine (Off-chain)**  
   - Runs as a dedicated Tokio task per market  
   - Uses `mpsc` and `oneshot` channels for message passing  
   - Produces fills/partial fills when a match is found  

4. **Backend submits the settlement instruction on-chain**  
   When a match occurs, the backend sends a transaction to the Anchor program **before persisting any state locally**.

5. **On-chain program executes and emits logs**  
   The Anchor program emits events after on-chain settlement.

6. **Event Listener processes logs**  
   Reads Anchor program logs and applies all confirmed updates to the database.

7. **Database stores canonical state**  
   Orders, fills, positions, and markets are only persisted after on-chain confirmation.

8. **Admin Panel**  
   - Create markets  
   - Resolve markets  
   - View market list and status  

---

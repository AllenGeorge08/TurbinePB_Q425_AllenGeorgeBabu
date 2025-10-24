# CINEPASS

## Project Name: CinePass

## Value Proposition

- Provides secure, transferable and verifiable movie tickets on the solana blockchain using NFT’s. Reduces fraud, enables secondary transfers, and give users a digital proof-of-attendance collectible.

## Product Market Fit

- CinePass targets urban,tech-savvy moviegoers who are comfortable with crypto wallets and want a better ticketing experience than centralized platforms. It is also beneficial for  theatres looking to reduce ticket fraud and simplify verification.

## Target User Profiles

### Primary User Persona

- Name: Aarav Sharma  
- Role: Regular Moviegoer  
- Goal: To Securely buy,own and transfer nft tickets without risk of fraud, and have a collectible NFT as proof of attendance.

### Secondary User Persona

- Name: Neha Mehta  
- Role: Theatre Manager  
- Goal: Verify Tickets Efficiently and reduce fraudulent entries.

## Acceptance Criteria

### Functionality
- Users can select movie, showtime and seat.
- Users can pay using SOL or SPL Tokens.
- NFT Ticket is minted and delivered to user’s wallet.

### NFT Attributes
- Includes metadata: movie name, showtime,theatre and seat number
- Includes a “used” flag to prevent double entry
- Includes a timestamp to expire the ticket after a certain time.

### User Interaction
- Wallet connection is seamless. (e.g Phantom,Metamask)
- Confirmation and ticket details are displayed post-payment

### Security
- NFT’s cannot be duplicated or reused for multiple entries
- On-chain verification ensures authenticity.

## Priority
High - Core user flow required for product viability.

## Technical Notes For Developers
- Integrate wallet authentication with solana.
- Implement smart contract for NFT minting.
- Track check-in status on-chain.
- Implement an expiry timestamp feature for every NFT Ticket.
- Consider transaction speed and low fees to make user experience easier.

### Dependencies
- Requires Solana network availability.
- Showtimes and seats must be pre-created on-chain
- Wallet onboarding needs UX guidance for new users

## User-story Id

**CINE-001**

**Persona:** Aarav Sharma, MovieGoer

**User Story:** As a moviegoer, I buy a movie ticket, I want a digital ticket that I can show at the theatre to gain entry, so that I’m guaranteed a valid seat without risk of fraud.

### Acceptance Criteria
- User selects showtime and seat
- Payment completes successfully
- User receives a digital ticket in their wallet or app
- Theatre staff can verify ticket validity at entry using a scanner
- Ticket cannot be reused after entry
- If payment or ticket delivery fails, the user is notified and funds are refunded.

---

![CinePass Architecture](/assets/CinePass.drawio.png)

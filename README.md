# Development

Run `mprocs` in the terminal. Ensure you have cargo, bun, and mprocs installed.

## TODO

- [x] Fine-tune UI theme (Just jumped for shadcn)
  - [x] Light
  - [x] Dark
- [x] Sidebar
  - [x] Add Federations
  - [x] Switch Federations
  - [x] Popover Options
      - [~] Remove Federations
      - [x] Invite / QR Code
  - [x] Polish
      - [x] Proper Loading States
      - [x] Responsive, toggleable
- [ ] Widgets
  - [x] Admin
    - [x] Info
    - [x] Meta
    - [x] Operations
    - [x] Denominations
  - [x] Lightning
    - [x] await
    - [x] gateways
    - [x] invoice
    - [x] pay
  - [x] Mint
    - [x] reissue
    - [x] spend
    - [x] validate
  - [x] Onchain
    - [x] await
    - [x] create_invoice

## Future Plans

- [ ] Github ci actions for building for multiple machines (Nix for deps), github releases
- [ ] Better loading states for await operations
- [ ] Split/Combine Notes
- [ ] Encode/Decode Notes
- [ ] Tweak Invoices
- [ ] Backup / Recovery with Seed Words
- [ ] Import Wallet with Seed Words (? Might clash with Fedi)
- [ ] Developer Mode (? Higher/Lower level client toggle)



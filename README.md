# Development

Run `mprocs` in the terminal. Ensure you have cargo, bun, and mprocs installed.

## TODO

- [x] Fine-tune UI theme (Just jumped for shadcn)
  - [x] Light
  - [x] Dark
- [ ] Sidebar
  - [x] Add Federations
  - [x] Switch Federations
  - [ ] Popover Options
      - [~] Remove Federations
      - [x] Invite / QR Code
  - [ ] Polish
      - [ ] Proper Loading States
      - [ ] Responsive, toggleable
- [ ] Widgets
  - [ ] Admin
    - [ ] Info
    - [ ] Operations
  - [ ] Lightning
    - [ ] await
    - [ ] claim_tweaks
    - [ ] gateways
    - [ ] invoice
    - [ ] mod
    - [ ] pay
    - [ ] tweak_invoice
  - [ ] Mint
    - [ ] combine
    - [ ] decode
    - [ ] encode
    - [ ] mod
    - [ ] reissue
    - [ ] spend
    - [ ] split
    - [ ] validate
  - [ ] Onchain
    - [ ] await
    - [ ] create_invoice
- [ ] Github ci actions for building for multiple machines (Nix for deps)

## Future Plans

- [ ] Backup / Recovery with Seed Words
- [ ] Import Wallet with Seed Words (? Might clash with Fedi)
- [ ] Developer Mode (? Higher/Lower level client toggle)

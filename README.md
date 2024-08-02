# Development

Run `mprocs` in the terminal. Ensure you have cargo, bun, and mprocs installed.

## TODO

- [ ] Better loading states for await operations
- [ ] Error handling for all operations (invalid codes, empty codes, etc)
- [ ] Github ci actions for building for multiple machines (Nix for deps), github releases
- [x] Better grid layout, max height for some widgets
- [ ] Split/Combine Notes
- [ ] Encode/Decode Notes
- [ ] Tweak Invoices
- [ ] Await invoice in ln payment modal
- [ ] Backup / Recovery / Importing
- [ ] Developer Mode (? Higher/Lower level client toggle)
- [ ] Denomination switch (sats/msats)
- [ ] Full keyboard nav
- [ ] Branding, find a better name

## Bugs to fix

- [ ] Multimint doesn't remove federations from the db
- [ ] Need to get traits merged into fedimint-clientd monorepo before release
- [x] Tailwind does't work in the release build

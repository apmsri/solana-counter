# Solana Counter

My first Solana program, built with Anchor in the Intro to Solana Program Development workshop by Texas Blockchain.

Each wallet gets its own counter stored at a PDA. Only the wallet that owns a counter can increment it.

## Deployed on devnet

Program ID `3o7GmW2VxQWJ6gFPWyUajWYXpXt1ic46tpVBNGrzyXxF`

[View it on Solana Explorer](https://explorer.solana.com/address/3o7GmW2VxQWJ6gFPWyUajWYXpXt1ic46tpVBNGrzyXxF?cluster=devnet)

## What's inside

| Folder | What it is |
| --- | --- |
| `counter/` | The Anchor program and its tests |
| `counter-client/` | A small TypeScript CLI that talks to the program on devnet |

The program has two instructions.

- `initialize` creates your counter at 0
- `increment` adds 1 and fails safely on overflow

Two checks stop anyone else from touching your counter. The PDA seeds must match the signer, and `has_one` checks the saved authority.

## Run the tests

You need Rust, the Solana CLI, Anchor 0.31.1, Node 20+ and Yarn.

```bash
cd counter
yarn install
anchor build
anchor test
```

Expected output

```
counter
  ✔ initializes at 0
  ✔ increments to 1
  ✔ increments again to 2
  ✔ cannot increment someone else's counter
4 passing
```

## Use the devnet client

The client uses your default Solana wallet at `~/.config/solana/id.json`.

```bash
cd counter-client
npm install
npm run cli -- status
npm run cli -- airdrop 1
npm run cli -- init
npm run cli -- increment
npm run cli -- get
npm run cli -- increment <someone-elses-wallet>   # fails with ConstraintSeeds
```

## Deploy it yourself

```bash
cd counter
solana config set --url devnet
solana airdrop 2
anchor build
anchor keys sync && anchor build   # first time only
anchor deploy --provider.cluster devnet
solana program show <PROGRAM_ID>
```

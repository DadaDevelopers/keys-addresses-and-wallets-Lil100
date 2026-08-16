[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/nhIMrY-0)
# assignment-2

Generate legacy addresses, bech32 addresses and bech32m addresses

What is the difference between hardened and non hardened keys
A non-hardened child key can be derived from the parent extended private key (xprv), and its corresponding child public key can also be derived from the parent extended public key (xpub). A hardened child key, however, can only be derived from the parent extended private key (xprv), not from the parent xpub.


Why should a wallet developer prefer deterministic wallets over non deterministic wallets

A wallet developer should prefer deterministic wallets because they generate all private keys and addresses from one recovery seed phrase. If a user loses their phone or changes devices, they can enter the same seed phrase into a compatible wallet and recover the same addresses and access any bitcoin held in them.

A non-deterministic wallet generates a separate random private key whenever it needs a new address. Each key must be backed up separately. If the user loses a private key that was not backed up, they cannot access the bitcoin sent to the address controlled by that key. Deterministic wallets are therefore easier to back up, recover, and manage safely.


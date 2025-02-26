## Chain guest ELF ID changelog
  * `9471af8ec35f9f6ffa891b504cc73735ebdc7488b613cdb0cf5cfb16d2b4627a` – Initial version deployed in December 2024
  * `18eac1fe344b5f8ce19ad5695ea43b5fe85dafb36c6158d51331adb4ea33eb88` – Teleport is a no-op on local-testnet (anvil) [#1721](https://github.com/vlayer-xyz/vlayer/pull/1721)
  * `907fa4c2e540a151e2053e66f638cb65932af0c9815324f16481d1922b4164aa` – History cleanup, seed with first deployed ELF ID [#1722](https://github.com/vlayer-xyz/vlayer/pull/1722)
  * `e1afa09f684c5a099b78b0a1425a7d9bec3ec8bd4d7a121f29a62f3bb0172573` – Teleport logging
  * `137a372bb2a4d011d502454e6f3ea45fc65094e8dd5ec9f3071d46281738f086` – Pin the versions of major dependencies
  * `eb712fcffa08e1b7777a74dae4634b91acdba28605655acf931144171e9a2b36` – Revm 19.0.0 -> 19.4.0
  * `11f35577f01a94b9de93158f6d80b331b75cb47f1643017f685216b842df33ad` – Use Nibbles instead of KeyNibbles in MPT
  * `f943ebf08ce7d8df4d97b57bde8f2f9c84ceadcafde7d2d0a8feb6278e7ce500` – Replace push_front function with Nibbles.join
  * `6d263e7c522d927ac813c3080764e23254699524e758486393163c1eeba761a7` – Convert Entry into Leaf instead Branch when the key is empty

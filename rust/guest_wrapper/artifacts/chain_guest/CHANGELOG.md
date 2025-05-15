## Chain guest ELF ID changelog
  * `9471af8ec35f9f6ffa891b504cc73735ebdc7488b613cdb0cf5cfb16d2b4627a` – Initial version deployed in December 2024
  * `18eac1fe344b5f8ce19ad5695ea43b5fe85dafb36c6158d51331adb4ea33eb88` – Teleport is a no-op on local-testnet (anvil) [#1721](https://github.com/vlayer-xyz/vlayer/pull/1721)
  * `907fa4c2e540a151e2053e66f638cb65932af0c9815324f16481d1922b4164aa` – History cleanup, seed with first deployed ELF ID [#1722](https://github.com/vlayer-xyz/vlayer/pull/1722)
  * `e1afa09f684c5a099b78b0a1425a7d9bec3ec8bd4d7a121f29a62f3bb0172573` – Teleport logging
  * `137a372bb2a4d011d502454e6f3ea45fc65094e8dd5ec9f3071d46281738f086` – Pin the versions of major dependencies
  * `eb712fcffa08e1b7777a74dae4634b91acdba28605655acf931144171e9a2b36` – Revm 19.0.0 -> 19.4.0
  * `11f35577f01a94b9de93158f6d80b331b75cb47f1643017f685216b842df33ad` – Use Nibbles instead of KeyNibbles in MPT
  * `f943ebf08ce7d8df4d97b57bde8f2f9c84ceadcafde7d2d0a8feb6278e7ce500` – Replace push_front function with Nibbles.join
  * `f3b12fb72b14d79384ef22555ea99f93b6d91908ffa2461186cfddaeb452c3c8` – Convert an Entry into a Leaf instead of a Branch when the key is empty
  * `166acfda7e752fe8e41324206accb169af629b0a80122dd6ca3955f053f06dcb` – Reset travel location after one call [#1890] (https://github.com/vlayer-xyz/vlayer/pull/1890)
  * `b732a05ac17d30e19bc66378e960d67290250da1c70f7c28a1c5be43e2670e7d` – Risc0 1.2.4 [#1911] (https://github.com/vlayer-xyz/vlayer/pull/1911)
  * `9200b482efbb414ff73203a2a1d4194ab3a9bea7114d5e01a6bf04ab306d7a6d` – Accelerated keccak in alloy-primitives
  * `b79154d432db45e7dd6e4ada75aca99837a8b25844a5041fd28b01f596bee43b` – Accelerated keccak in MPT
  * `3b4d5f70a9fe3cf01638faf6f1a8d6e6079ce5f089f2a3dd875f74c107e643f2` – Risc0 2.0.0
  * `82d44d66876bf5cc8ff56a3c8fc91270b13c61623d9e6fc3a566ab0a7a126468` – Added functionality and a crate dependency in `common` [#2138] (https://github.com/vlayer-xyz/vlayer/pull/2138)
  * `4af5b2998ca397f906dc0cffbe13937dde04c1cdd833fb460dd81d2815468896` – Rust 1.85.0 - [#2150] (https://github.com/vlayer-xyz/vlayer/pull/2150)
  * `605a504383da17012e2af35e674bb0fb8b4d9ceb2714217d3618f43c3d4c6ded` – hashbrown 0.15.0 -> 0.15.1 - [#2011] (https://github.com/vlayer-xyz/vlayer/pull/2011)
  * `3dedd7024c3462f3f326489b27ff1843f30f03afff71cdd88ccd24f81fbe8f67` – Deny panics globally [#2199](https://github.com/vlayer-xyz/vlayer/pull/2199)
  * `304957ee9fdecb19ec65f8633d40a926aacd142b81c4cfd43d42ca6742f67130` – Update `chain_specs.toml` [#2218](https://github.com/vlayer-xyz/vlayer/pull/2218)
  * `d47c721762613c905c892b615c3496fc172ea7d5f7fc29ae9900a4f0b5478dd3` – Risc0 2.0.2 - [#2279](https://github.com/vlayer-xyz/vlayer/pull/2279)
  * `202097073e66ede38bf441f210b93e7983bd2473b1680f945ebacf41bf1d5ed8` – Update `AnchorStateRegistry` addresses for unichain and base chains

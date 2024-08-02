#!/bin/bash
./target/release/schain benchmark pallet \
--chain dev \
--pallet pallet_smultisig \
--extrinsic '*' \
--steps 50 \
--repeat 20 \
--output pallets/smultisig/src/weights.rs

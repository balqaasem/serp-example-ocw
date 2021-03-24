# Serp Offchain Worker Example Module

The Serp Offchain Worker Example: A simple pallet demonstrating
concepts, APIs and structures common to most offchain workers.

## Overview

In this example we are going to build a very simplistic, naive and definitely NOT
production-ready oracle for DNAR/USD, SETT/USD and JUSD/USD prices.

The BTC here represents DNAR since DNAR is not yet listed in the market.
The USDT here represents SETT since SETT is not yet listed in the market.
DAI here represents JUSD, since JUSD is not yet listed in the market.
Offchain Worker (OCW) will be triggered after every block, fetch the current price
and prepare either signed or unsigned transaction to feed the result back on chain.
The on-chain logic will simply aggregate the results and store last `64` values to compute
the average price.
Additional logic in OCW is put in place to prevent spamming the network with both signed
and unsigned transactions, and custom `UnsignedValidator` makes sure that there is only
one unsigned transaction floating in the network.

License: Unlicense

# Based Anchor Migration Tests

Requires `based-anchor` to build and run. To do this:

1. Clone down Based Anchor: `git clone https://github.com/deanmlittle/based-anchor.git`
2. Build the CLI: `make build-cli`
3. Put based anchor in your path: `cp -f ./target/release/anchor /Users/elon/.cargo/bin/based-anchor`
rename the binary the outputted binary to run your tests, ie: `based-anchor test --skip-local-validator`

GG
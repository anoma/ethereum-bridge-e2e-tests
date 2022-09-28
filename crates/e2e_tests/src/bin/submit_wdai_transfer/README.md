# submit_wdai_transfer

This test submits a TransferToEthereum event to a sole validator, and then verifies that:

- the transfer was acted on (i.e. wDAI was minted for the receiver)
- the wDAI can then be transferred by the receiver to another established Namada address

In this test, the receiver is an established account (rather than an implicit account).

/// send connection requests to peers (for net discovery and opening channels)
/// Channels: open channels between sender and receiver and let them send and receive on that
/// without synchronization until: channel is closed or certain amount is reached. Requires
/// initial transaction with a certain amount that is saved to the chain
/// i.e. only safe first commitment and closing/overflowing commitments
/// Take care of this nodes PGP key pair and signing

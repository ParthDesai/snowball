# Snowball (Work in progress)
Once it is complete, Snowball will contain generic implementation of snowball algorithm described [here](https://ipfs.io/ipfs/QmUy4jh5mGNZvLkjies1RWM4YuvJh5o2FYopNPVYwrRVGV) with batteries included.

# Overview
This library will provide plug and play support for embedding snowball in any rust project and since the implementation is generic it can be used to achieve consensus on any data as long as said data is serializable. Specifically, it will provide following:
- Main snowball loop
- Snowball query handler
- Generic implementations for snowball network interface, query object, and node sampler among other things.

# Roadmap
- [x] Add initial version of traits
- [x] Implement initial version of snowball loop
- [ ] Publish initial documentation
- [ ] Implement initial version of snowball query handler
- [ ] Write unit tests for snowball loop and fix bugs
- [ ] Write unit tests for snowball query handler and fix bugs
- [ ] Provide default implementation of traits
- [ ] Write unit tests for default implementation

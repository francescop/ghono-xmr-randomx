# What is

A simple Monero RandomX Miner, inspired by [this project](https://github.com/kazcw/powhasher).

# Usage

Edit the included `config.toml` to suit your needs (pool address, number of threads, etc), then run:

```
RUST_LOG=debug ghono-xmr-randomx -c config.toml
```

While the hasher is running, press Enter to get statistics.

# What is it?

This is a simple CLI miner for Monero ([randomx](https://github.com/tevador/RandomX))
and a modified version of the [cn-stratum](https://github.com/kazcw/cn-stratum) pool client - included in this repo
for convenience.

It uses [this project](https://github.com/tari-project/randomx-rs) for the randomx bindings.

# Supported platforms

This has been tested on linux so far.
Will test it on other platforms if time permits.

# Suggested configuration

```toml
[randomx]
cores = [ 0 , 1 , 2 ] # number of phisical cores - 1
hard_aes = true
jit = true
argon2_avx2 = true
full_mem = true
large_pages = true # if hugepages are enabled, see comments below
argon2_ssse3 = true # if your processor supports sse3 instructions
```

## Hugepages

On linux, enable hugepages with:

```sh
sudo sysctl -w vm.nr_hugepages=3750
```

The value (in this case 3750), is the number of cores times 1250.

Ex: 3 cores * 1250 = 3750

# Donations

This project does not automatically donates to any address.
Consider donating if you want to support this project.

```
xmr: 45d15JymvBiEJ3e682FeDYYDytnTGrxH4Dv1jdZg7rgqHFMrvwt5tSYZhRKyv55Ny265HsVANH4p6LLpbH3hxiKg6ha8Jir
eth: 0x881Fba89B9f8c2d042F03e3c1C5E0eF07f9a4fF6
btc: 1Lo6hUPV3v7DATfMAcdvnt4t5ujsdLAzBX
```

In my list I have:

[ ] rust randomx implementation - right now it uses a c library binding
[ ] tests
[ ] reporting
[ ] ci/cd integration
[ ] monitoring


NOTE: THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

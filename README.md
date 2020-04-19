# ARCHIVAL NOTICE

I built this to help me get my server back online reliably, but after I got it working on the subdomain I used for testing, I realized that OVH's DynHost only works for subdomains. So, doesn't work for my usecase. Then I thought well, what about switching to using their regular API, and I was nearly done with that, but then I looked at how they are authenticating requests, and honestly, fuck that. There is no reason why they need to reinvent the wheel there, but they did. I'm switching to Cloudflare.

# ovh-dynhost

ovh-dynhost is a dynamic DNS daemon for OVH's DynHost feature.

## Installation

You can install ovh-dynhost like any other rust package.

```bash
git clone https://git.jcg.re/jcgruenhage/ovh-dynhost.git
cd ovh-dynhost
cargo install --path .
```

## Usage

The basic mode of operation is `ovh-dynhost /path/to/config.toml`.
For more usage information, run `ovh-dynhost --help`.

You can find a sample configuration file in this repo, it's called `config.sample.toml`.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update logging, documentation and config samples accordingly.

## License
[AGPL-3.0-only](https://choosealicense.com/licenses/agpl-3.0-only/)

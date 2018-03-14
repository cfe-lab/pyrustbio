# pyrustbio
Python bindings for rust-bio

## Installation

1. Install [Rust] itself.

        curl https://sh.rustup.rs -sSf | sh

2. Check that rust installed correctly. You may need to start a new shell session.

        rustc --version

3. Switch to the nightly build to enable experimental features.

        rustup install nightly
        rustup default nightly

4. Install `setuptools_rust`.

        pip install setuptools_rust

5. Install this Python binding. It requires at least Python 3.5.

        cd /path/to/pyrustbio
        pip install .

    OR

        pip install https://github.com/cfe-lab/pyrustbio

[Rust]: https://www.rust-lang.org/en-US/install.html
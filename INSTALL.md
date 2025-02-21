# CANPi Maintenance Web Application

This functionality is distributed as a Debian package, as a compressed source tarball,
and contained in an Apt Repository.

- The package and the sources are available as part of a release on github
(`emthornber/canpi-web-app-ssr`)

- The Apt repository is available at [MERG-DEV Apt
Repository](https://emthornber.github.io/rpirepo).  Visiting the site displays
instructions on how to setup the apt configuration to access the MERG-DEV
repository along with a downloadable script to carry out those instructions.

## Compiling

The source code is written in Rust and the build uses Python3 library `tomli` to
extract key values from the Cargo.toml file.

The .deb package is built using Easy Package Manager (EPM) (`emthornber/epm` tag
`v5.0.1rc2`) which is built from source using the usual autotools incantation
```
./configure
make
sudo make install
```

## Installation

After a successful build there is a Debian package (and a portable tarball) in `./package` e.g.
```
canpi-web-app-ssr-0.1.8-linux-6.6-armhf.deb
canpi-web-app-ssr-0.1.8-linux-6.6-armhf.tar.gz
```
which can be installed using `apt`
```
sudo apt install ./package/canpi-web-app-ssr-0.1.8-linux-6.6-armhf.deb
```

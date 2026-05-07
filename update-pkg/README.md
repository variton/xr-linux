# Update-pkg 
Tool that allows to update ubuntu packages.
It pipes the list of upgradable packages with apt install.

## Build
Use the following command line to build the tool:

*in debug*
```
cargo build 

```

*in release*
```
cargo build --release

```

*in release for target CPU*
```
RUSTFLAGS="-C target-cpu=native" cargo build --release

```

## Update ubuntu packages
Use the following command line to install upgradable packages.

Updates the updatable ubuntu packages.
```
./update-pkg

```

## To run tests
todo :prepare a simple test with a dry run

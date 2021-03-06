# vrf-rs-jna

A JNA lib of vrf-rs for Java.

## How to build library ?

* install dependencies

```Bash
# macOS
brew install openssl@1.1

# Arch Linux
sudo pacman -S pkg-config openssl

# Debian and Ubuntu
sudo apt-get update && sudo apt-get upgrade
sudo apt-get install pkg-config libssl-dev

# Fedora
sudo dnf install pkg-config openssl-devel
```

* install rust

```Bash
# install
curl https://sh.rustup.rs -sSf | sh

# 增加环境变量
source $HOME/.cargo/env
```

* build

```Bash
# build
cargo build --release

# list library
ls target/release/libvrfjna*
```

## How to use in Java ?


* Java Dependency

```xml
<dependency>
  <groupId>net.java.dev.jna</groupId>
  <artifactId>jna</artifactId>
  <version>4.5.2</version>
</dependency>
```

* Java Code

```Java
import com.sun.jna.Library;
import com.sun.jna.Native;

/**
 *
 */

public class JavaCallRust {

    public interface VRFLib extends Library {

        VRFLib INSTANCE = (VRFLib) Native.loadLibrary("vrfjna", VRFLib.class);

        String prove(String sk, String preSeed);

        boolean verify(String pk, String preSeed, String pi);
    }

    public static void main(String[] args) {
        String sk = "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721";
        String preSeed = "1ffff";
        String pk = "032c8c31fc9f990c6b55e3865a184a4ce50e09481f2eaeb3e60ec1cea13a6ae645";

        String prove = VRFLib.INSTANCE.prove(sk, preSeed);
        System.out.println(prove);
        System.out.println(VRFLib.INSTANCE.verify(pk, preSeed, prove));
    }
}
```

## Thanks

[java-rust-example](https://github.com/drrb/java-rust-example)     
[vrf-rs](https://github.com/witnet/vrf-rs)

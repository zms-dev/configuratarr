## services\.configuratarr\.enable



Whether to enable Configuratarr declarative configuration service\.



*Type:*
boolean



*Default:*

```nix
false
```



*Example:*

```nix
true
```

*Declared by:*
 - [/nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos\.nix](file:///nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos.nix)



## services\.configuratarr\.package



The configuratarr package to use\.



*Type:*
package



*Default:*

```nix
<derivation configuratarr-0.1.0>
```

*Declared by:*
 - [/nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos\.nix](file:///nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos.nix)



## services\.configuratarr\.configFile

Path to the configuratarr\.yaml configuration file\.



*Type:*
absolute path

*Declared by:*
 - [/nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos\.nix](file:///nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos.nix)



## services\.configuratarr\.prune



Whether to prune server-side resources that are not declared in the config file\.



*Type:*
boolean



*Default:*

```nix
false
```

*Declared by:*
 - [/nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos\.nix](file:///nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos.nix)



## services\.configuratarr\.wait



Wait for target applications to be online before running synchronization\.



*Type:*
boolean



*Default:*

```nix
true
```

*Declared by:*
 - [/nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos\.nix](file:///nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos.nix)



## services\.configuratarr\.waitTimeout



Timeout in seconds to wait for each application to become online\.



*Type:*
unsigned integer, meaning >=0



*Default:*

```nix
30
```

*Declared by:*
 - [/nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos\.nix](file:///nix/store/24fmgdpjhawsniy2rcwh5z1xlgcpv7s7-source/modules/nixos.nix)



{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    openssl
    pkg-config
  ];
  shellHook = ''
    export OPEN_SSL_DIR=${openssl.out}/lib
  '';
}

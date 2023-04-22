{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    buildInputs = [
      geckodriver
      firefox
      pkg-config
    ];
  }

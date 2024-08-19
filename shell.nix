{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc 
    cargo
    openssl.dev
    pkg-config
  ];
}

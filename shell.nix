{ pkgs ? import <nixpkgs> {} }:
with pkgs;

mkShell {
  nativeBuildInputs = [
    cargo
    rust-analyzer rustc rustfmt
  ];
}

{
  description = "Add replay gain to files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
  pkgs = import nixpkgs { system = "x86_64-linux"; }; 
in {

    packages.x86_64-linux.add-replay-gain = pkgs.callPackage ./pkg.nix {};
    packages.aarch64.add-replay-gain = pkgs.callPackage ./pkg.nix {};

    nixosModules.add-replay-gain = import ./module.nix self;

    hydraJobs = {
      inherit (self) packages;
    };   
  };
}

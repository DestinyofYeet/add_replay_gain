{
  description = "Add replay gain to files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
  pkgs = import nixpkgs { system = "x86_64-linux"; }; 
in {

    packages.x86_64-linux.add-replay-gain = nixpkgs.lib.callPackage ./pkg.nix {};

    nixosModules.add-replay-gain = import ./module.nix self;
    
  };
}

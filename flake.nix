{
  description = "Add replay gain to files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {

    packages.x86_64-linux.add-replay-gain = nixpkgs.lib.callPackage ./pkg.nix {};

    nixosModules.add-replay-gain = import ./module.nix self;
    
  };
}

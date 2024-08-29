{
  description = "Add replay gain to files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
  
  forAllSystems = function:
    nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
    ]
    (system: function (import nixpkgs { inherit system; }));

  mkMultiple = function: 
    nixpkgs.lib.genAttrs [
      "add-replay-gain"
      "default"
    ] (name: function (name));
in {

    packages = forAllSystems (pkgs: 
      {
        add-replay-gain = pkgs.callPackage ./pkg.nix {};
        default = pkgs.callPackage ./pkg.nix {};
      }
    );

    nixosModules.add-replay-gain = import ./module.nix self;

    hydraJobs = {
      inherit (self) packages;
    };   
  };
}

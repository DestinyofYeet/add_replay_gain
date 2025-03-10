{
  description = "Add replay gain to files";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };

      forAllSystems = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          # "aarch64-linux"
        ] (system: function (import nixpkgs { inherit system; }));

    in {

      packages = forAllSystems (pkgs: rec {
        default = pkgs.callPackage ./pkg.nix { };
        add-replay-gain = default;
      });

      nixosModules.add-replay-gain = import ./module.nix self;

      hydraJobs = { inherit (self) packages; };
    };
}

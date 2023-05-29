{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default (import rust-overlay)];
        };

        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.64.0";
          rustChannel = "stable";
          packageFun = import ./Cargo.nix;
        };

      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rustVersion.override { extensions = [ "rust-src" ]; }) # for rust
            cargo-nextest
            pkg-config
          ];
        };
    });
}

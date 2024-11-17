{
  # testing flake: nix develop --unset PATH && cargo run

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay/stable";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        target = "x86_64-unknown-linux-gnu";
        toolchainOverride = {
          extensions = [ "rust-src" ];
          targets = [ target ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          nativeBuildInputs = [
            # (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override toolchainOverride))
            (pkgs.rust-bin.stable.latest.default.override toolchainOverride)
            # (rust-bin.stable."1.73.0".default.override toolchainOverride)
          ];
        };
      }
    );
}

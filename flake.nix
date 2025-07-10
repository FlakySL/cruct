{
  description = "Flake configuration file for cruct development.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;

        # Determine the Rust toolchain
        toolchain =
          with fenix.packages.${system};
          combine [
            stable.rustc
            stable.rust-src
            stable.cargo
            complete.rustfmt
            stable.clippy
            stable.rust-analyzer
            stable.llvm-tools-preview
          ];

        # Override the toolchain in crane
        craneLib = crane.overrideToolchain toolchain;
      in
      {
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            toolchain
            cargo-llvm-cov
            llvmPackages_19.libllvm
            grcov
            cargo-nextest
          ];

          env = {
            LAZYVIM_RUST_DIAGNOSTICS = "bacon-ls";
          };
        };
      }
    );
}

{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs23_11.url = "github:NixOS/nixpkgs/90bd1b26e23760742fdcb6152369919098f05417";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,

    nixpkgs23_11,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        pkgs23_11 = import nixpkgs23_11 {inherit system;};
        dependencies = with pkgs; [wireplumber pkgs23_11.wireplumber libcxx.dev glibc.dev clang llvm glib pipewire libclang];
        naersk-lib = pkgs.callPackage naersk {};
      in {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = [pkgs.pkg-config];
          buildInputs = dependencies;
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [cargo rustc rustfmt pre-commit rustPackages.clippy] ++ dependencies;
            LIBCLANG_PATH = "${libclang.lib}/lib";
            nativeBuildInputs = [pkgs.pkg-config];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}

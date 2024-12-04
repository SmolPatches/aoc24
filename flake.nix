{
  inputs = {
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils, ... }@inputs: utils.lib.eachDefaultSystem (system:
    let
      # toml info
      installWithMusl = true;
      cargoTOML = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      packageVersion =  "1.0";
      rustVersion = "1.83.0";
      pname = "aoc24";
      rustPkg = pkgs.rust-bin.stable."${rustVersion}".default.override {
        extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        targets = [];
      };
      # platform packages
      macPackages = pkgs.lib.optionals pkgs.stdenv.isDarwin [];
      linuxPackages = pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [

      ]);
      overlays = [ (import inputs.rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      cargoDeps = pkgs.rustPlatform.importCargoLock {
        lockFile = ./Cargo.lock;
      };
      rustPlatform = pkgs.makeRustPlatform {
        rustc = rustPkg;
        cargo = rustPkg;
      };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          macPackages
          linuxPackages
          rustPkg
          cargoDeps
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      };

      defaultPackage = rustPlatform.buildRustPackage {
        pname = pname;
        name = pname;
        version = packageVersion;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        checkFlags = [
          #this test breaks Read Only FS sandbox
          #"--skip=cli::tests::parse_env_filter_directives"
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        nativeBuildInputs = (with pkgs;[
          # common packages
        ]) ++ macPackages ++ linuxPackages;
        src = ./.;
      };
    }
  );
}

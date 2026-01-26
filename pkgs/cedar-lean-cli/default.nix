{
  lib,
  fetchFromGitHub,
  rustPlatform,
  makeWrapper,
  cedar,
  cedar-lean,
  lean,
  protobuf,
  cvc5,
}:

rustPlatform.buildRustPackage {
  pname = "cedar-lean-cli";
  inherit (cedar) version;

  src = fetchFromGitHub {
    owner = "cedar-policy";
    repo = "cedar-spec";
    tag = "v${cedar.version}";
    hash = "sha256-2vLBBTECZ5w3PqQuIkDtXzjR9rIbJo/swQxZ0YJhQlI=";
  };

  buildAndTestSubdir = "cedar-lean-cli";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  cargoPatches = [
    ./build-rs.patch
  ];

  nativeBuildInputs = [
    makeWrapper
    lean.lean-all
    protobuf
  ];

  postUnpack = ''
    cp ${./Cargo.lock} $sourceRoot/Cargo.lock

    cp -r ${cedar.src} $sourceRoot/cedar
    chmod -R u+w $sourceRoot/cedar

    mkdir -p $sourceRoot/cedar-lean/.lake/build
    ln -s ${cedar-lean}/lib $sourceRoot/cedar-lean/.lake/build/lib

    mkdir -p $sourceRoot/cedar-lean/.lake/packages/batteries/.lake/build
    ln -s ${cedar-lean}/lib $sourceRoot/cedar-lean/.lake/packages/batteries/.lake/build/lib
  '';

  preBuild = ''
    export LEAN_LIB_DIR=$(lean --print-libdir)
    export RUSTFLAGS="-L ${cedar-lean}/lib -L $LEAN_LIB_DIR"
    export CEDAR_PROTO_DIR="$NIX_BUILD_TOP/source/cedar/cedar-policy/protobuf_schema"
  '';

  doCheck = false;

  postFixup = ''
    wrapProgram $out/bin/cedar-lean-cli \
      --set CVC5 ${cvc5}/bin/cvc5
  '';

  meta = {
    description = "Lean formalization of, and proofs about, Cedar";
    homepage = "https://github.com/cedar-policy/cedar-spec";
    license = lib.licenses.asl20;
    mainProgram = "cedar-lean-cli";
  };
}

{
  lib,
  stdenv,
  writeText,
  fetchFromGitHub,
  cedar,
  lean,
}:

let
  src = fetchFromGitHub {
    owner = "cedar-policy";
    repo = "cedar-spec";
    tag = "v${cedar.version}";
    hash = "sha256-2vLBBTECZ5w3PqQuIkDtXzjR9rIbJo/swQxZ0YJhQlI=";
  };

  batteries = fetchFromGitHub {
    owner = "leanprover-community";
    repo = "batteries";
    rev = "v${lean.version}";
    hash = "sha256-iO0z7Us5pDSfw61tijLcAnNaq48kh7gOjydZVY57Oxo=";
  };

  lakeManifest = writeText "lake-manifest.json" (
    builtins.toJSON {
      name = "Cedar";
      version = "1.1.0";
      lakeDir = ".lake";
      packagesDir = ".lake/packages";
      packages = [
        {
          type = "git";
          name = "batteries";
          url = "https://github.com/leanprover-community/batteries";
          rev = batteries.rev;
          inputRev = "v${lean.version}";
          inherited = false;
          configFile = "lakefile.lean";
          manifestFile = "lake-manifest.json";
          scope = "leanprover-community";
        }
      ];
    }
  );

  packageOverrides = writeText "package-overrides.json" (
    builtins.toJSON {
      name = "Cedar";
      version = "1.1.0";
      lakeDir = ".lake";
      packagesDir = ".lake/packages";
      packages = [
        {
          type = "path";
          name = "batteries";
          inherited = false;
          dir = ".lake/packages/batteries";
        }
      ];
    }
  );
in
stdenv.mkDerivation {
  pname = "cedar-lean";
  inherit (cedar) version;

  src = "${src}/cedar-lean";

  nativeBuildInputs = [
    lean.lean-all
  ];

  configurePhase = ''
    cp ${lakeManifest} lake-manifest.json
    mkdir -p .lake/packages
    cp ${packageOverrides} .lake/package-overrides.json
    cp -r ${batteries} .lake/packages/batteries
    chmod -R +w .lake/packages
  '';

  buildPhase = ''
    lake build Cedar:static Protobuf:static CedarProto:static Cedar.SymCC:static CedarFFI:static Batteries:static
  '';

  installPhase = ''
    mkdir -p $out/lib
    cp .lake/build/lib/*.a .lake/packages/batteries/.lake/build/lib/*.a $out/lib
  '';

  meta = {
    description = "Lean formalization of, and proofs about, Cedar";
    homepage = "https://github.com/cedar-policy/cedar-spec";
    license = lib.licenses.asl20;
  };
}

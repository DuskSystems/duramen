{
  lib,
  rustPlatform,
  fetchFromGitHub,
  protobuf,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cedar";
  version = "4.9.0";

  src = fetchFromGitHub {
    owner = "cedar-policy";
    repo = "cedar";
    tag = "v${finalAttrs.version}";
    hash = "sha256-rwbzh2+fRf4gDcbftEn9Ln/Hlj0VWQoK3K64k2o/x9k=";
  };

  cargoHash = "sha256-rVL0vKg0JQol9DmelCBLQSAJ4TprIAmGdxEnfEByIbU=";

  nativeBuildInputs = [
    protobuf
  ];

  cargoBuildFlags = [
    "--bin"
    "cedar"
    "--features"
    "experimental"
  ];

  cargoTestFlags = [
    "--bin"
    "cedar"
    "--features"
    "experimental"
  ];

  meta = {
    description = "Implementation of the Cedar Policy Language";
    homepage = "https://github.com/cedar-policy/cedar";
    changelog = "https://github.com/cedar-policy/cedar/releases/tag/v${finalAttrs.version}";
    license = lib.licenses.asl20;
    mainProgram = "cedar";
  };
})

{
  rustPlatform,
  fetchFromGitHub,
  lib,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cedar";
  version = "4.8.2";

  src = fetchFromGitHub {
    owner = "cedar-policy";
    repo = "cedar";
    tag = "v${finalAttrs.version}";
    hash = "sha256-ap5c6+RVGLpa4IKA0S49dvDqs6g1T/L2o90MWzuRpA4=";
  };

  cargoHash = "sha256-l9/wBxgIJNMdkEJFs6SAGzXv/O5ZvXTCUpzWj45zza8=";
  cargoBuildFlags = [
    "--package=cedar-policy-cli"
    "--features=experimental"
  ];

  doCheck = false;

  meta = {
    description = "Cedar is a language for writing and enforcing authorization policies in your applications";
    homepage = "https://www.cedarpolicy.com";
    changelog = "https://github.com/cedar-policy/cedar/releases/tag/v${finalAttrs.version}";
    license = lib.licenses.asl20;
    mainProgram = "cedar";
  };
})

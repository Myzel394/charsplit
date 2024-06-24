{ lib
, stdenv
, rustPlatform
, pkg-config
, libiconv
, installShellFiles
, makeWrapper
, darwin
}:

rustPlatform.buildRustPackage {
  pname = "charsplit";
  version = "0.1.0";

  src = ./.;
  cargoHash = "sha256-Y6Lo3e5l1lk5H5cpzVj5PrxxDEQbMLtvOAAgCzC8g/s=";

  nativeBuildInputs = [ pkg-config installShellFiles makeWrapper ];

  buildInputs = lib.optionals stdenv.isDarwin [ libiconv darwin.apple_sdk.frameworks.Security ];

  doInstallCheck = true;
  installCheckPhase = ''
    runHook preInstallCheck

    echo 'Hello World' | $out/bin/charsplit 
    echo 'привет' | $out/bin/charsplit 

    runHook postInstallCheck
  '';

  meta = {
    description = "Split a string into its bytes and characters";
    homepage = "https://github.com/Myzel394/charsplit";
    mainProgram = "charsplit";
  };
}

{ 
  rustPlatform,
  lib,
  fetchFromGitHub,
  pkgs,
}: 

rustPlatform.buildRustPackage rec {
  pname = "add_replay_gain_to_files";
  version = "1.7";

  src = fetchFromGitHub {
    owner = "DestinyofYeet";
    repo = "add_replay_gain";
    rev = "fbcee0534694ad057bdc273cd5b7476ad718a13c";
    hash = "sha256-N8Bkg5IuIkQnQhTUbKEEzaCZMLlHxZqTnDJ6CR6wxmI=";
  };

  cargoHash = "sha256-e0bgEAmJNKRWVjSMlb3qPIjjk16UDNGbpBGKXzWKHbE=";

  nativeBuildInputs = with pkgs; [
    pkg-config
    openssl.dev
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  meta = with lib; {
    description = "A tool to automatically add replay gain to mp3 and flac files";
    license = licenses.gpl2;
    # maintainers = [ maintainers.DestinyofYeet ];
    platforms = platforms.all;
  };
}

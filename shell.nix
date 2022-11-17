let pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
    # dependencies
    buildInputs = with pkgs; [ 
        cargo
        rustc
        pkg-config
        openssl
    ];
}

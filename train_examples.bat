@echo off
echo Sigmos Training Examples with Organized Output
echo.
echo Standard Training (to trained/ directory):
echo ./target/release/sigmos-train -n wildcat -d textfiles -o output-sigs/trained/wildcat.sig -r 0.05 --style all --continuous --all-personalities --programming --cuda --verbose
echo.
echo Compressed Training (saves ~97%% space):
echo ./target/release/sigmos-train -n wildcat -d textfiles -o output-sigs/compressed/wildcat.sig -r 0.05 --style all --continuous --all-personalities --programming --cuda --verbose --compress
echo.
echo Experimental Training (for testing):
echo ./target/release/sigmos-train -n experiment -d textfiles -o output-sigs/experimental/experiment.sig -r 0.05 --style all --continuous --all-personalities --programming --cuda --verbose
echo.
echo Prompting Examples:
echo ./target/release/sigmos-prompt --sigel output-sigs/trained/wildcat.sig
echo ./target/release/sigmos-prompt --sigel output-sigs/compressed/wildcat.sig.gz
echo ./target/release/sigmos-prompt --sigel output-sigs/experimental/experiment.sig
echo.
pause
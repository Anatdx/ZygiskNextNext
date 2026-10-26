shopt -s globstar

cd .github
$(go env GOBIN)/yamlfmt .
cd ..
$(go env GOBIN)/yamlfmt module.yaml

cd injector
cargo fmt
cd ..

cd loader
git ls-files | grep -E "\.(cpp|h|hpp|c)$" | xargs clang-format -style=Google -i
git restore src/external/lzma src/external/znn_shadowhook
cd ..

cd webui
npx prettier --write "src/**/*.{vue,ts,js,css}"
cd ..

cd kpm
git ls-files | grep -E "\.(cpp|h|hpp|c)$" | xargs clang-format -style=Google -i
git restore external
cd ..

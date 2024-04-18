cd /d "%~dp0"
cd game2048
wasm-pack build --target web
cd ..
xcopy "%CD%\game2048\pkg" "%CD%\static" /s /Y
copy "%CD%\game2048\index.html" "%CD%\static"
cargo run
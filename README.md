# Sokoban на WASM

## Установка зависимостей для Rust и WASM

1. Установить компилятор Rust с системой сборки Cargo согласно [инструкциям](https://www.rust-lang.org/ru/tools/install).
2. Установить поддержку WebAssembly в качестве целевой платформы для Rust
    ```
    $ rustup target add wasm32-unknown-unknown
    ```
3. Установить `wasm-pack`, запустив [инсталлятор](https://rustwasm.github.io/wasm-pack/installer/) из командной строки.
4. [Установить `npm`](https://nodejs.org/ru/), если он ещё не установлен (все веб-ориентированные вещи в Rust часто связаны с 
    обычными для javascript-разработчика инструментами). 
5. Выполнить команду сборки, чтобы `cargo` заранее скачал и собрал все основные зависимости:
    ```
   $ cargo build
    ```

## Далее

... будут дальнейшие действия
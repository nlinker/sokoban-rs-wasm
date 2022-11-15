# Sokoban на WASM

## Установка зависимостей для Rust и WASM

1. Установить компилятор Rust с системой сборки Cargo согласно [инструкциям](https://www.rust-lang.org/ru/tools/install). 
    Если вы на Windows и не используете WSL, то придётся скачать Visual Studio Installer и установить VC++2017 и 
    Windows 10 SDK, это потребует скачать примерно 2.5Гб (!). В Linux, MacOS и Windows/WSL нужно будет скачать только 
    инструменты для Rust, это примерно 230Мб.

2. Установить поддержку WebAssembly в качестве целевой платформы для Rust
    ```
    $ rustup target add wasm32-unknown-unknown
    ```
3. Установить `wasm-pack`, запустив [инсталлятор](https://rustwasm.github.io/wasm-pack/installer/). Для Windows есть `exe`, 
   для остальных систем - команда в командной строке. 

4. [Установить `npm`](https://nodejs.org/ru/), если он ещё не установлен (все веб-ориентированные вещи в Rust часто связаны с 
    обычными для javascript-разработчика инструментами).

5. Выполнить команду сборки, чтобы `cargo` заранее скачал и собрал все основные зависимости:
    ```
   $ cargo build
    ```

6. Также трудно будет без IDE, самые популярные 2 опции:
   1. [VS Code](https://code.visualstudio.com/) с плагином [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) 
   2. [Intellij Idea Community](https://www.jetbrains.com/idea/download/) или [CLion](https://www.jetbrains.com/clion/)
      с плагином [Rust](https://www.jetbrains.com/rust/). (CLion платный, но есть испытательный период). 


## Демо

![Sokoban demo](img/demo.mp4)

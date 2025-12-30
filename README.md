# <kbd>Ctrl</kbd> + <kbd>TeX</kbd>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![CI](https://github.com/Yendric/ctrl.tex/actions/workflows/ci.yml/badge.svg)
![Issues](https://img.shields.io/github/issues/Yendric/ctrl.tex)

A small background utility that converts selected LaTeX code into Unicode characters in-place. Default hotkey is <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>L</kbd>, but it can be changed by modifying `main.rs`.

## Usage

> This project requires you to have Rust installed.

1. Clone: `git clone git@github.com:Yendric/ctrl.tex`
2. Run with `cargo run --release` (The app runs in the background)
3. Select some LaTeX text in any text input
4. Press `Ctrl + Shift + L`
5. The selection will be replaced by its Unicode equivalent

## Workings

The app listens for the hotkey globally. When pressed, it simulates a `Ctrl + C` to copy the selected text to the clipboard, processes it, and then simulates a `Ctrl + V` to paste the converted text back.

## Contribute

Feel free to create an issue/PR if you have suggestions or find mistakes.

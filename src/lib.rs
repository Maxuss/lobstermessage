#![feature(test)]

extern crate test;

use logos::Lexer;
use crate::component::{AsComponent, Component};
use crate::tokens::{MessageToken, Parser};

pub mod tokens;
pub mod component;


#[cfg(test)]
mod tests {
    #![allow(soft_unstable)]

    use test::Bencher;
    use crate::tokens::{MessageToken, Parser};
    use logos::{Logos, Lexer};
    use crate::{Component, lobster, placeholder_lobster};

    #[test]
    fn test_lexer() {
        let mut lexer: Lexer<MessageToken> = MessageToken
            ::lexer("<#AABBCC>Hex text<reset>Stop hex text");

        while let Some(tk) = lexer.next() {
            println!("{:?}", tk)
        }
    }

    #[test]
    fn test_parser() {
        let lexer: Lexer<MessageToken> = MessageToken::lexer("<red>Red text");
        let mut parser = Parser::new(lexer);

        while let Ok(_) = parser.advance() {
            // no-op
        }
        let out = parser.finish();
        println!("{}", serde_json::to_string(&out).unwrap());
    }

    #[test]
    fn test_placeholders() {
        let lobster = placeholder_lobster("Before placeholder, <replace_me> Stuff after placeholder. <another>", [
            ("replace_me", lobster("<aqua>This is a <dark_aqua>placeholder!<reset>")),
            ("another", lobster("<gold><bold>Another placeholder!"))
        ]);

        println!("{}", serde_json::to_string(&lobster).unwrap());
    }

    #[test]
    fn test_flattening() {
        let mut message = lobster("<red>Some message<blue> Even more message <green>Green message ").append(Component::translate::<&str, Component>("some.message.translate", None));

        println!("{}", message.flatten())
    }

    #[bench]
    fn benchmark_lobster(bencher: &mut Bencher) {
        bencher.iter(|| {
            test::black_box(lobster("<red>Red text <green>Green text <italic><yellow>Yellow italic text. <bold>BOLD. <red>Red text"))
        })
    }
}

pub fn lobster<S: Into<String>>(msg: S) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<MessageToken> = MessageToken::lexer(&st);
    let parser = Parser::new(lexer);

    parser.parse()
}

pub fn placeholder_lobster<S: Into<String>, C: AsComponent + Sized, const N: usize>(msg: S, placeholders: [(S, C); N]) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<MessageToken> = MessageToken::lexer(&st);
    let mut parser = Parser::new(lexer);
    for (k, v) in placeholders {
        parser.placeholder(k, v);
    }

    parser.parse()
}
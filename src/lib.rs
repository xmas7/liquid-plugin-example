extern crate liquid;
use liquid::Renderable;
use liquid::Context;
use liquid::Template;
use liquid::LiquidOptions;
use liquid::Token;
use liquid::lexer::Element;
use liquid::Error;
use liquid::parser;

struct Shout{
    inner: Template,
}

impl Renderable for Shout {
    fn render(&self, context: &mut Context) -> Result<Option<String>, Error> {
        Ok(try!(self.inner.render(context)).map(|content| content.to_uppercase()))
    }
}

pub fn initialize_shout(_tag_name: &str,
                        _arguments: &[Token],
                        tokens: Vec<Element>,
                        options: &LiquidOptions)
                        -> Result<Box<Renderable>, Error> {
    Ok(Box::new(Shout { inner: Template::new(try!(parser::parse(&tokens, options))) }))
}

#[test]
fn it_works() {
    use std::default::Default;

    let mut options: LiquidOptions = Default::default();
    options.blocks.insert("shout".to_string(), Box::new(initialize_shout));
    let template = liquid::parse("{% shout %}Liquid!{% endshout %}", options).unwrap();
    let mut data = Context::new();
    let output = template.render(&mut data);
    assert_eq!(output.unwrap(), Some("LIQUID!".to_string()));
}

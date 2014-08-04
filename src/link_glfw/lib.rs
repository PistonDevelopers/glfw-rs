// Copyright 2014 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use std::gc::{Gc, GC};
use std::io::Command;
use std::mem;
use std::str;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::parse::token::InternedString;

#[plugin_registrar]
pub fn registrar(reg: &mut rustc::plugin::Registry) {
    reg.register_syntax_extension(token::intern("link_glfw"),
                                  base::ItemModifier(expand));
}

enum Meta {
    MetaName(InternedString),
    MetaNameValue(InternedString, ast::Lit_),
    MetaList(InternedString, Vec<Meta>),
}

impl Meta {
    fn to_meta_item(self, context: &mut base::ExtCtxt, span: codemap::Span) -> Gc<ast::MetaItem> {
        match self {
            MetaName(name) => context.meta_word(span, name),
            MetaNameValue(name, value) => context.meta_name_value(span, name, value),
            MetaList(name, list) => {
                let meta_items = list.move_iter().map(|meta| {
                    meta.to_meta_item(context, span)
                }).collect();
                context.meta_list(span, name, meta_items)
            },
        }
    }

    fn to_attribute(self, context: &mut base::ExtCtxt, span: codemap::Span) -> ast::Attribute {
        let meta = self.to_meta_item(context, span);
        context.attribute(span, meta)
    }

    fn link(name: InternedString, kind: Option<InternedString>) -> Meta {
        let mut name_meta = MetaNameValue(InternedString::new("name"),
                                          ast::LitStr(name, ast::CookedStr));
        MetaList(InternedString::new("link"), match kind {
            Some(kind) => {
                let kind_meta = MetaNameValue(InternedString::new("kind"),
                                              ast::LitStr(kind, ast::CookedStr));
                vec![name_meta, kind_meta]
            },
            None => {
                vec![name_meta]
            },
        })
    }
}

pub fn expand(context: &mut base::ExtCtxt, span: codemap::Span, meta_item: Gc<ast::MetaItem>,
              item: Gc<ast::Item>) -> Gc<ast::Item> {
    let out = Command::new("pkg-config")
        .arg("--static")
        .arg("--libs-only-l")
        .arg("--libs-only-other")
        .arg("glfw3")
        .output();
    match out {
        Ok(out) => {
            if out.status.success() {
                let mut item = (*item).clone();
                str::from_utf8(out.output.as_slice()).map(|output| {
                    let mut expect_framework = false;
                    for word in output.words() {
                        if word.starts_with("-l") {
                            item.attrs.push(Meta::link(
                                token::intern_and_get_ident(word.slice_from(2)),
                                None,
                            ).to_attribute(context, span));
                        } else if expect_framework {
                            expect_framework = false;
                            item.attrs.push(Meta::link(
                                token::intern_and_get_ident(word),
                                Some(InternedString::new("framework")),
                            ).to_attribute(context, span));
                        } else if word.starts_with("-framework") {
                            expect_framework = true;
                        }
                    }
                });
                box (GC) item
            } else {
                context.span_err(span, format!("error returned by \
                    `pkg-config`: ({})", out.status).as_slice());
                item
            }
        },
        Err(e) => {
            context.span_err(span, format!("io error: {}", e).as_slice());
            item
        },
    }
}

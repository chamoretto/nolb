#![feature(
    async_closure,
    type_alias_impl_trait,
    generic_associated_types,
    min_specialization,
    const_trait_impl,
    decl_macro,
    let_else,
    bool_to_option,
    total_cmp,
    const_option,
    const_result,
    associated_type_defaults
)]
#![deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::manual_ok_or,
    clippy::redundant_else,
    clippy::redundant_closure_for_method_calls,
    clippy::same_functions_in_if_condition
)]

extern crate core;

mod commands;
mod config;
mod errors;
mod parsing;
mod utility;

use crate::commands::config::Config;
use commands::main_app::MainApp;
use structopt::StructOpt;

fn main() {
    // нужно пытаться запустить демона agent с параметрами не серверного admin устройства и оповещать об этом
    // типа, нужно же как-то соединять cli-утилиту и внешне-внутреннюю сеть агентов
    // как вариант - через dialoguer спрашивать об этом юзера, типа на какой порт повесить, и хранить это где-нибудь
    // в /usr или AppData, хз.
    let opt = MainApp::from_args();

    // println!("{:?}", &opt);

    match opt {
        MainApp::Config(c) => match c {
            Config::Check(cc) => {
                cc.check().unwrap();
            },
            Config::Apply(ca) => {
                todo!()
            },
            Config::Current => {
                todo!()
            },
        },
        MainApp::Status(s) => {
            todo!()
        },
    }
}

#[test]
fn test_smth() {
    // TODO: test uom
    use uom::si::information::Information;
}

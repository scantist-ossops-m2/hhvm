/*
   +----------------------------------------------------------------------+
   | HipHop for PHP                                                       |
   +----------------------------------------------------------------------+
   | Copyright (c) 2010-present Facebook, Inc. (http://www.facebook.com)  |
   +----------------------------------------------------------------------+
   | This source file is subject to version 3.01 of the PHP license,      |
   | that is bundled with this package in the file LICENSE, and is        |
   | available through the world-wide-web at the following url:           |
   | http://www.php.net/license/3_01.txt                                  |
   | If you did not receive a copy of the PHP license and are unable to   |
   | obtain it through the world-wide-web, please send a note to          |
   | license@php.net so we can mail you a copy immediately.               |
   +----------------------------------------------------------------------+
*/

/*

This is the script that parses the configuration specification. The format is
following:

```
Any comments until we hit the first section

# SectionName

  Optional section comment paragraph that should be indented 2 spaces

- Type ConfigName[ = DefaultValue], Owner[, feature1|feature2...]

  Optional config comment paragraph that should be indented 2 spaces

```

Supported Types can be found in ConfigType

The ConfigName always need to be prefixed with the `SectionName.`. This is
done to make it easier to search

If no default value is applied you need to implement a custom function that
returns the default value. You can look at php7-impl.cpp for an example.

If the Owner is unknown use `UNKNOWN` as the owner.

Possible features are:
- private, will make the config private so it can only be used as default value
  for other configs in the same section
- globaldata, if the config should be part of RepoGlobalData
- unitcacheflag, if the config should be part of the unit cache hash key
- repooptionsflag(name[, systemlibdefault]), if the config can be set in
  hhvmconfig.hdf inside the repo that contains the hack code. Use systemlibdefault
  incase the normal default is not a constant value
- compileroption(name), if the config is something the compiler needs
- lookuppath(name), use if you need to read from a different name then the configs
  name
- nobind, doesn't call Config::Bind which means it can't be set using hdf or
  command line
- postprocess, if the config needs a PostProcess method to process the value
  after it been read from the config
- staticdefault(value), if you need to set the default of the static variable
  to something other than the default for that type

*/

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use clap::ValueEnum;
use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::character::complete::none_of;
use nom::character::complete::not_line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::all_consuming;
use nom::combinator::cut;
use nom::combinator::eof;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::error::context;
use nom::error::convert_error;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::error::VerboseError;
use nom::multi::count;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigValue {
    Const(String),
    Name(String),
}

impl ConfigValue {
    fn string(&self, section_names: &Vec<String>) -> String {
        match self {
            ConfigValue::Const(v) => v.to_owned(),
            ConfigValue::Name(v) => {
                for sn in section_names {
                    if v.starts_with(sn) {
                        return format!(
                            "Cfg::{}::{}",
                            sn.replace('.', ""),
                            v.strip_prefix(&(sn.clone() + ".")).unwrap()
                        );
                    }
                }
                panic!("Unknown prefix {}", v);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigFeature {
    Private,
    GlobalData,
    UnitCacheFlag,
    RepoOptionsFlag(String, Option<String>),
    CompilerOption(String),
    LookupPath(String),
    NoBind,
    PostProcess,
    StaticDefault(String),
}

#[derive(Debug, PartialEq, Default)]
pub struct ConfigFeatureRepoOptionFlag {
    pub prefix: String,
    pub default_value: Option<String>,
}

#[derive(Debug, PartialEq, Default)]
pub struct ConfigFeatures {
    pub is_repo_specific: bool,
    pub is_request_level: bool,
    pub is_private: bool,
    pub is_global_data: bool,
    pub is_unit_cache_flag: bool,
    pub repo_options_flag: Option<ConfigFeatureRepoOptionFlag>,
    pub compiler_option: Option<String>,
    pub lookup_path: Option<String>,
    pub has_no_bind: bool,
    pub has_post_process: bool,
    pub static_default: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    type_: ConfigType,
    pub name: String,
    pub default_value: Option<ConfigValue>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub features: ConfigFeatures,
}

impl Config {
    fn shortname(&self, section_name: &str) -> String {
        self.name
            .strip_prefix(&(section_name.to_owned() + "."))
            .unwrap()
            .replace('.', "")
    }

    fn hdf_path(&self, section: &ConfigSection) -> String {
        if let Some(lookup_path) = &self.features.lookup_path {
            return lookup_path.clone();
        }
        match &section.prefix {
            Some(prefix) => format!(
                "{}{}",
                prefix,
                self.name
                    .strip_prefix(&(section.name.to_owned() + "."))
                    .unwrap()
            ),
            None => self.name.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigType {
    Bool,
    Int,
    Double,
    Int8t,
    Int16t,
    Int32t,
    Int64t,
    UInt8t,
    UInt16t,
    UInt32t,
    UInt64t,
    StdString,
    StdVectorStdString,
    StdSetStdString,
    StdMapStdStringStdString,
    BoostFlatSetStdString,
}

impl ConfigType {
    fn str(&self) -> &str {
        match *self {
            ConfigType::Bool => "bool",
            ConfigType::Int => "int",
            ConfigType::Double => "double",
            ConfigType::Int8t => "int8_t",
            ConfigType::Int16t => "int16_t",
            ConfigType::Int32t => "int32_t",
            ConfigType::Int64t => "int64_t",
            ConfigType::UInt8t => "uint8_t",
            ConfigType::UInt16t => "uint16_t",
            ConfigType::UInt32t => "uint32_t",
            ConfigType::UInt64t => "uint64_t",
            ConfigType::StdString => "std::string",
            ConfigType::StdVectorStdString => "std::vector<std::string>",
            ConfigType::StdSetStdString => "std::set<std::string>",
            ConfigType::StdMapStdStringStdString => "std::map<std::string,std::string>",
            ConfigType::BoostFlatSetStdString => "boost::container::flat_set<std::string>",
        }
    }

    fn includes(&self) -> Vec<&str> {
        match *self {
            ConfigType::Bool | ConfigType::Double | ConfigType::Int => vec![],
            ConfigType::Int8t
            | ConfigType::Int16t
            | ConfigType::Int32t
            | ConfigType::Int64t
            | ConfigType::UInt8t
            | ConfigType::UInt16t
            | ConfigType::UInt32t
            | ConfigType::UInt64t => vec!["cstdint"],
            ConfigType::StdString => vec!["string"],
            ConfigType::StdVectorStdString => vec!["vector", "string"],
            ConfigType::StdSetStdString => vec!["set", "string"],
            ConfigType::StdMapStdStringStdString => vec!["map", "string"],
            ConfigType::BoostFlatSetStdString => {
                vec!["boost/container/flat_set.hpp", "string"]
            }
        }
    }

    fn default(&self) -> &str {
        match *self {
            ConfigType::Bool => "false",
            ConfigType::Double => "0.0",
            ConfigType::Int
            | ConfigType::Int8t
            | ConfigType::Int16t
            | ConfigType::Int32t
            | ConfigType::Int64t
            | ConfigType::UInt8t
            | ConfigType::UInt16t
            | ConfigType::UInt32t
            | ConfigType::UInt64t => "0",
            ConfigType::StdString => "\"\"",
            ConfigType::StdVectorStdString => "{}",
            ConfigType::StdSetStdString => "{}",
            ConfigType::StdMapStdStringStdString => "{}",
            ConfigType::BoostFlatSetStdString => "{}",
        }
    }

    fn repo_option(&self) -> &str {
        match *self {
            ConfigType::StdVectorStdString => "Cfg::StringVector",
            ConfigType::StdSetStdString => "Cfg::StringSet",
            ConfigType::StdMapStdStringStdString => "Cfg::StringStringMap",
            ConfigType::BoostFlatSetStdString => "Cfg::StringBoostFlatSet",
            _ => self.str(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigSection {
    pub name: String,
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub configs: Vec<Config>,
}

impl ConfigSection {
    fn shortname(&self) -> String {
        self.name.replace('.', "")
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputType {
    Defs,
    Loader,
}

#[derive(Debug, Parser)]
#[clap(name = "HHVM Generate Configs")]
struct Arguments {
    #[clap(value_enum)]
    output_type: OutputType,
    output_dir: PathBuf,
    input: PathBuf,
}

fn parse_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ConfigType, E> {
    preceded(
        space0,
        alt((
            value(ConfigType::Bool, tag("bool")),
            value(ConfigType::Int8t, tag("int8_t")),
            value(ConfigType::Int16t, tag("int16_t")),
            value(ConfigType::Int32t, tag("int32_t")),
            value(ConfigType::Int64t, tag("int64_t")),
            value(ConfigType::Int, tag("int")),
            value(ConfigType::Double, tag("double")),
            value(ConfigType::UInt8t, tag("uint8_t")),
            value(ConfigType::UInt16t, tag("uint16_t")),
            value(ConfigType::UInt32t, tag("uint32_t")),
            value(ConfigType::UInt64t, tag("uint64_t")),
            value(ConfigType::StdString, tag("std::string")),
            value(
                ConfigType::StdVectorStdString,
                tag("std::vector<std::string>"),
            ),
            value(ConfigType::StdSetStdString, tag("std::set<std::string>")),
            value(
                ConfigType::StdMapStdStringStdString,
                tag("std::map<std::string,std::string>"),
            ),
            value(
                ConfigType::BoostFlatSetStdString,
                tag("boost::container::flat_set<std::string>"),
            ),
        )),
    )(input)
}

fn parse_name<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "parse name",
        recognize(tuple((
            alphanumeric1,
            many0(tuple((tag("_"), alphanumeric1))),
        ))),
    )(input)
}

fn parse_name_with_dot<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "parse name with dot",
        preceded(
            space0,
            recognize(tuple((parse_name, many1(preceded(tag("."), parse_name))))),
        ),
    )(input)
}

fn parse_name_with_optional_dot<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "parse name with optional dot",
        preceded(
            space0,
            recognize(tuple((parse_name, many0(preceded(tag("."), parse_name))))),
        ),
    )(input)
}

fn parse_partial_name<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "parse partial name",
        preceded(
            space0,
            recognize(tuple((parse_name, many0(alt((tag("."), parse_name)))))),
        ),
    )(input)
}

fn parse_string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(delimited(
        tag("\""),
        opt(escaped(none_of(r#"\""#), '\\', one_of(r#"\""#))),
        tag("\""),
    ))(input)
}

fn parse_num<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    alt((
        recognize(delimited(
            tuple((space0, tag("("))),
            parse_num_expr,
            tuple((space0, tag(")"))),
        )),
        recognize(tuple((
            space0,
            opt(alt((tag("-"), tag("+")))),
            alt((
                tag("INT_MAX"),
                tag("INT64_MAX"),
                recognize(tuple((
                    alt((
                        recognize(tuple((digit1, opt(preceded(tag("."), digit1))))),
                        recognize(preceded(tag("."), digit1)),
                    )),
                    opt(alt((tag("LL"), tag("ll"), tag("ULL"), tag("ull")))),
                ))),
            )),
        ))),
    ))(input)
}

fn parse_num_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(tuple((
        parse_num,
        opt(many0(preceded(
            tuple((
                space0,
                alt((tag("*"), tag("+"), tag("-"), tag("/"), tag("<<"), tag(">>"))),
            )),
            parse_num,
        ))),
    )))(input)
}

fn parse_bool<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(alt((tag("true"), tag("false"))))(input)
}

fn parse_constant_value<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    alt((tag("{}"), parse_string, parse_num_expr, parse_bool))(input)
}

fn parse_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ConfigValue, E> {
    alt((
        map(parse_constant_value, |v| ConfigValue::Const(v.to_string())),
        map(parse_name_with_dot, |v: &str| {
            ConfigValue::Name(v.to_string())
        }),
    ))(input)
}

fn parse_default_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Option<ConfigValue>, E> {
    context(
        "default value",
        opt(preceded(
            tuple((space0, tag("="), space1)),
            cut(parse_value),
        )),
    )(input)
}

fn parse_owner<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Option<&'a str>, E> {
    map(
        preceded(tuple((space0, tag(","), space0)), alphanumeric1),
        |s: &str| {
            if s == "UNKNOWN" {
                return None;
            }
            Some(s)
        },
    )(input)
}

fn parse_features<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ConfigFeatures, E> {
    map(
        opt(preceded(
            tuple((space0, tag(","), space0)),
            separated_list1(
                tag("|"),
                alt((
                    value(ConfigFeature::Private, tag("private")),
                    value(ConfigFeature::GlobalData, tag("globaldata")),
                    value(ConfigFeature::UnitCacheFlag, tag("unitcacheflag")),
                    map(
                        delimited(
                            tag("repooptionsflag("),
                            tuple((
                                parse_name_with_optional_dot,
                                opt(preceded(
                                    tuple((space0, tag(","), space0)),
                                    parse_constant_value,
                                )),
                            )),
                            tag(")"),
                        ),
                        |(name, default_value)| {
                            ConfigFeature::RepoOptionsFlag(
                                name.to_string(),
                                default_value.map(|v| v.to_string()),
                            )
                        },
                    ),
                    value(ConfigFeature::NoBind, tag("nobind")),
                    map(
                        delimited(
                            tag("compileroption("),
                            parse_name_with_optional_dot,
                            tag(")"),
                        ),
                        |name| ConfigFeature::CompilerOption(name.to_string()),
                    ),
                    map(
                        delimited(tag("lookuppath("), parse_name_with_optional_dot, tag(")")),
                        |name| ConfigFeature::LookupPath(name.to_string()),
                    ),
                    value(ConfigFeature::PostProcess, tag("postprocess")),
                    map(
                        delimited(tag("staticdefault("), parse_constant_value, tag(")")),
                        |name| ConfigFeature::StaticDefault(name.to_string()),
                    ),
                )),
            ),
        )),
        |v| {
            let mut features = ConfigFeatures::default();
            for f in v.unwrap_or(vec![]).into_iter() {
                match f {
                    ConfigFeature::Private => features.is_private = true,
                    ConfigFeature::GlobalData => features.is_global_data = true,
                    ConfigFeature::UnitCacheFlag => features.is_unit_cache_flag = true,
                    ConfigFeature::RepoOptionsFlag(prefix, default_value) => {
                        features.repo_options_flag = Some(ConfigFeatureRepoOptionFlag {
                            prefix,
                            default_value,
                        })
                    }
                    ConfigFeature::CompilerOption(name) => features.compiler_option = Some(name),
                    ConfigFeature::LookupPath(name) => features.lookup_path = Some(name),
                    ConfigFeature::NoBind => features.has_no_bind = true,
                    ConfigFeature::PostProcess => features.has_post_process = true,
                    ConfigFeature::StaticDefault(name) => features.static_default = Some(name),
                }
            }
            features
        },
    )(input)
}

fn parse_description<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Option<String>, E> {
    context(
        "description",
        alt((
            preceded(
                count(newline, 2),
                opt(map(
                    many1(delimited(tag("  "), not_line_ending, newline)),
                    |s: Vec<&str>| s.join("\n"),
                )),
            ),
            value(None, many0(newline)),
        )),
    )(input)
}

fn parse_config<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Config, E> {
    context(
        "config",
        map(
            preceded(
                tuple((opt(newline), tag("-"), space1)),
                tuple((
                    parse_type,
                    parse_name_with_dot,
                    parse_default_value,
                    parse_owner,
                    parse_features,
                    parse_description,
                )),
            ),
            |(type_, name, default_value, owner, features, description)| Config {
                type_,
                name: name.to_string(),
                default_value,
                owner: owner.map(|s| s.to_string()),
                features,
                description,
            },
        ),
    )(input)
}

fn parse_section<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ConfigSection, E> {
    context(
        "section",
        map(
            tuple((
                preceded(
                    tuple((opt(newline), tag("#"), space1)),
                    parse_name_with_optional_dot,
                ),
                opt(delimited(
                    tuple((space1, tag("("))),
                    parse_partial_name,
                    tag(")"),
                )),
                parse_description,
                many1(parse_config),
            )),
            |(name, prefix, description, configs)| ConfigSection {
                name: name.to_string(),
                prefix: prefix.map(|s| s.to_string()),
                description,
                configs,
            },
        ),
    )(input)
}

fn parse_option_doc<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<ConfigSection>, E> {
    all_consuming(preceded(
        take_until("#"),
        map(many_till(parse_section, eof), |(cs, _r)| cs),
    ))(input)
}

fn generate_defs(sections: Vec<ConfigSection>, output_dir: PathBuf) {
    let section_names = sections.iter().map(|s| s.name.clone()).collect::<Vec<_>>();

    for section in sections.iter() {
        let section_shortname = section.shortname();
        let lower_section_shortname = section_shortname.to_lowercase();

        // [section].h file
        let mut public_defs = vec![];
        let mut private_defs = vec![];
        let mut includes = vec![];
        for config in section.configs.iter() {
            let shortname = config.shortname(&section.name);
            if config.features.repo_options_flag.is_none() {
                for include in config.type_.includes() {
                    includes.push(include);
                }
                if config.features.is_private {
                    private_defs.push(format!("  static {} {};", config.type_.str(), shortname));
                } else {
                    public_defs.push(format!("  static {} {};", config.type_.str(), shortname));
                }
            }
        }

        let mut unique_includes = includes
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        unique_includes.sort();
        let unique_include_paths = unique_includes
            .into_iter()
            .map(|p| format!("#include <{}>", p))
            .collect::<Vec<_>>();

        let h_content = format!(
            r#"#pragma once

{}

namespace HPHP::Cfg {{

struct {} {{

  friend struct {}Loader;

{}

private:
{}

}};

}} // namespace HPHP::Cfg
"#,
            unique_include_paths.join("\n"),
            section_shortname,
            section_shortname,
            public_defs.join("\n"),
            private_defs.join("\n"),
        );
        let mut h_output_file = output_dir.clone();
        h_output_file.push(format!("{}.h", lower_section_shortname));
        fs::write(h_output_file, h_content).unwrap();

        // [section].cpp file
        let mut defs = vec![];
        for config in section.configs.iter() {
            let shortname = config.shortname(&section.name);
            if config.features.repo_options_flag.is_none() {
                defs.push(format!(
                    "{} {}::{} = {};",
                    config.type_.str(),
                    section_shortname,
                    shortname,
                    if config.features.has_no_bind {
                        config
                            .default_value
                            .as_ref()
                            .unwrap()
                            .string(&section_names)
                    } else if let Some(d) = &config.features.static_default {
                        d.to_string()
                    } else {
                        config.type_.default().to_string()
                    },
                ));
            }
        }

        let cpp_content = format!(
            r#"#include "{}.h"

namespace HPHP::Cfg {{

{}

}} // namespace HPHP::Cfg
"#,
            lower_section_shortname,
            defs.join("\n"),
        );
        let mut cpp_output_file = output_dir.clone();
        cpp_output_file.push(format!("{}.cpp", lower_section_shortname));
        fs::write(cpp_output_file, cpp_content).unwrap();
    }
}

fn generate_loader(sections: Vec<ConfigSection>, output_dir: PathBuf) {
    let mut repo_global_data = vec![];
    let mut unit_cache_flags = vec![];
    let mut repo_options_includes = vec![];
    let mut repo_options_flags = vec![];
    let mut repo_options_sections = vec![];

    let section_names = sections.iter().map(|s| s.name.clone()).collect::<Vec<_>>();

    for section in sections.iter() {
        let section_shortname = section.shortname();
        let lower_section_shortname = section_shortname.to_lowercase();

        let has_repo_option_flags = section
            .configs
            .iter()
            .any(|c| c.features.repo_options_flag.is_some());

        if has_repo_option_flags {
            repo_options_includes.push(format!(
                r#"#include "hphp/runtime/base/configs/{}-loader.h""#,
                lower_section_shortname
            ));
            repo_options_sections.push(format!("  S({}Loader)\\", section_shortname));
        }

        // [section]-loader.h
        let mut public_methods = vec![];
        let mut private_methods = vec![];
        for config in section.configs.iter() {
            let shortname = config.shortname(&section.name);
            if config.default_value.is_none() {
                public_methods.push(format!(
                    "  static {} {}Default();",
                    config.type_.str(),
                    shortname
                ));
            }
            if config.features.has_post_process {
                private_methods.push(format!(
                    "  static void {}PostProcess({}& value);",
                    shortname,
                    config.type_.str(),
                ));
            }
        }

        public_methods
            .push("  static void Load(const IniSettingMap& ini, const Hdf& config);".to_string());
        public_methods.push("  static std::string Debug();".to_string());
        if has_repo_option_flags {
            public_methods.push("  static void GetRepoOptionsFlags(RepoOptionsFlags& flags, const IniSettingMap& ini, const Hdf& config);".to_string());
            public_methods.push("  static void GetRepoOptionsFlagsFromConfig(RepoOptionsFlags& flags, const Hdf& config, const RepoOptionsFlags& default_flags);".to_string());
            public_methods.push(
                "  static void GetRepoOptionsFlagsForSystemlib(RepoOptionsFlags& flags);"
                    .to_string(),
            );
        }

        let has_compiler_option = section
            .configs
            .iter()
            .any(|c| c.features.compiler_option.is_some());
        if has_compiler_option {
            public_methods.push(
                "  static void LoadForCompiler(const IniSettingMap& ini, const Hdf& config);"
                    .to_string(),
            );
        }

        let h_content = format!(
            r#"#pragma once

#include <string>

namespace HPHP {{

struct IniSettingMap;
struct Hdf;
struct RepoOptionsFlags;

namespace Cfg {{

struct {}Loader {{

{}

private:
{}

}};

}} // namespace Cfg

}} // namespace HPHP

"#,
            section_shortname,
            public_methods.join("\n"),
            private_methods.join("\n"),
        );

        let mut h_output_file = output_dir.clone();
        h_output_file.push(format!("{}-loader.h", lower_section_shortname));
        fs::write(h_output_file, h_content).unwrap();

        // [section]-loader.cpp file
        let mut bind_calls = vec![];
        let mut debug_calls = vec![];
        let mut compiler_option_calls = vec![];
        let mut repo_options_flags_calls = vec![];
        let mut repo_options_flags_from_config_calls = vec![];
        let mut repo_options_flags_for_systemlib_calls = vec![];
        for config in section.configs.iter() {
            let shortname = config.shortname(&section.name);
            let default_value = match &config.default_value {
                Some(v) => v.string(&section_names),
                None => format!("{}Default()", shortname),
            };
            if let Some(repo_option_flag) = &config.features.repo_options_flag {
                repo_options_flags_calls.push(format!(
                    r#"  Config::Bind(flags.{}, ini, config, "{}", {});"#,
                    shortname,
                    config.hdf_path(section),
                    default_value,
                ));
                repo_options_flags_from_config_calls.push(format!(
                    r#"  hdfExtract(config, "{}.{}", flags.{}, default_flags.{});"#,
                    repo_option_flag.prefix,
                    config.hdf_path(section),
                    shortname,
                    shortname
                ));
                repo_options_flags_for_systemlib_calls.push(format!(
                    r#"  flags.{} = {};"#,
                    shortname,
                    repo_option_flag.default_value.as_ref().unwrap_or_else(|| {
                        if let Some(ConfigValue::Const(s)) = &config.default_value {
                            return s;
                        }
                        panic!("repooptionflags need a constant default value");
                    })
                ));
                debug_calls.push(format!(
                    r#"  fmt::format_to(std::back_inserter(out), "Cfg::{}::{} = Repo Option Flag so UNKNOWN!\n");"#,
                    section_shortname, shortname,
                ));
            } else {
                if !config.features.has_no_bind {
                    bind_calls.push(format!(
                        r#"  Config::Bind(Cfg::{}::{}, ini, config, "{}", {});"#,
                        section_shortname,
                        shortname,
                        config.hdf_path(section),
                        default_value
                    ));
                    if config.features.has_post_process {
                        bind_calls.push(format!(
                            r#"  {}PostProcess(Cfg::{}::{});"#,
                            shortname, section_shortname, shortname
                        ));
                    }
                }
                debug_calls.push(format!(
                    r#"  fmt::format_to(std::back_inserter(out), "Cfg::{}::{} = {{}}\n", Cfg::{}::{});"#,
                    section_shortname, shortname, section_shortname, shortname,
                ));
                if let Some(compiler_option_name) = &config.features.compiler_option {
                    compiler_option_calls.push(format!(
                        r#"  Config::Bind(Cfg::{}::{}, ini, config, "{}", {});"#,
                        section_shortname, shortname, compiler_option_name, default_value,
                    ));
                }
            }
        }

        let mut repo_options_methods = String::new();
        if has_repo_option_flags {
            repo_options_methods = format!(
                r#"void {}Loader::GetRepoOptionsFlags(RepoOptionsFlags& flags, const IniSetting::Map& ini, const Hdf& config) {{
{}
}}

void {}Loader::GetRepoOptionsFlagsFromConfig(RepoOptionsFlags& flags, const Hdf& config, const RepoOptionsFlags& default_flags) {{
{}
}}

void {}Loader::GetRepoOptionsFlagsForSystemlib(RepoOptionsFlags& flags) {{
{}
}}

"#,
                section_shortname,
                repo_options_flags_calls.join("\n"),
                section_shortname,
                repo_options_flags_from_config_calls.join("\n"),
                section_shortname,
                repo_options_flags_for_systemlib_calls.join("\n"),
            );
        }

        let mut compiler_options_method = String::new();
        if has_compiler_option {
            compiler_options_method = format!(
                r#"void {}Loader::LoadForCompiler(const IniSettingMap& ini, const Hdf& config) {{
{}
}}

"#,
                section_shortname,
                compiler_option_calls.join("\n"),
            );
        }

        let cpp_content = format!(
            r#"#include "hphp/runtime/base/configs/{}-loader.h"

#include "hphp/runtime/base/ini-setting.h"
#include "hphp/runtime/base/runtime-option.h"
#include "hphp/util/configs/{}.h"
#include "hphp/util/hdf-extract.h"

#include <fmt/core.h>
#include <fmt/ranges.h>

namespace HPHP::Cfg {{

void {}Loader::Load(const IniSetting::Map& ini, const Hdf& config) {{
{}
}}

std::string {}Loader::Debug() {{
  std::string out;
{}
  return out;
}}

{}{}}} // namespace HPHP::Cfg
"#,
            lower_section_shortname,
            lower_section_shortname,
            section_shortname,
            bind_calls.join("\n"),
            section_shortname,
            debug_calls.join("\n"),
            &repo_options_methods,
            &compiler_options_method
        );

        let mut cpp_output_file = output_dir.clone();
        cpp_output_file.push(format!("{}-loader.cpp", lower_section_shortname));
        fs::write(cpp_output_file, cpp_content).unwrap();

        // macro files
        for config in section.configs.iter() {
            let shortname = config.shortname(&section.name);
            if config.features.is_global_data {
                repo_global_data.push(format!(
                    "  C(Cfg::{}::{}, {}_{}, {})\\",
                    section_shortname,
                    shortname,
                    section_shortname,
                    shortname,
                    &config.type_.str(),
                ));
            }
            if config.features.is_unit_cache_flag {
                unit_cache_flags.push(format!(
                    "  C(Cfg::{}::{}, {}_{}, {})\\",
                    section_shortname,
                    shortname,
                    section_shortname,
                    shortname,
                    &config.type_.str(),
                ));
            }
            if config.features.repo_options_flag.is_some() {
                repo_options_flags.push(format!(
                    "  C({}, {})\\",
                    &config.type_.repo_option(),
                    shortname
                ));
            }
        }
    }

    // repo-global-data-generated.h
    let repo_global_data_content = format!(
        r#"#pragma once
#define CONFIGS_FOR_REPOGLOBALDATA() \
{}

"#,
        repo_global_data.join("\n")
    );
    let mut repo_global_data_file = output_dir.clone();
    repo_global_data_file.push("repo-global-data-generated.h");
    fs::write(repo_global_data_file, repo_global_data_content).unwrap();

    // unit-cache-generated
    let unit_cache_content = format!(
        r#"#pragma once

#define CONFIGS_FOR_UNITCACHEFLAGS() \
{}

"#,
        unit_cache_flags.join("\n")
    );
    let mut unit_cache_file = output_dir.clone();
    unit_cache_file.push("unit-cache-generated.h");
    fs::write(unit_cache_file, unit_cache_content).unwrap();

    // repo-option-flags-generated.h
    let repo_options_flags_content = format!(
        r#"#pragma once

#include "hphp/runtime/base/configs/configs.h"
{}

#define CONFIGS_FOR_REPOOPTIONSFLAGS() \
{}

#define SECTIONS_FOR_REPOOPTIONSFLAGS() \
{}

"#,
        repo_options_includes.join("\n"),
        repo_options_flags.join("\n"),
        repo_options_sections.join("\n"),
    );
    let mut repo_option_flags_file = output_dir.clone();
    repo_option_flags_file.push("repo-options-flags-generated.h");
    fs::write(repo_option_flags_file, repo_options_flags_content).unwrap();

    let mut config_load_includes = vec![];
    let mut config_load_calls = vec![];
    let mut config_load_compiler_calls = vec![];
    let mut repo_options_flags_calls = vec![];
    let mut repo_options_flags_from_config_calls = vec![];
    let mut repo_options_flags_for_systemlib_calls = vec![];

    for section in sections.iter() {
        let section_shortname = section.shortname();
        let lower_section_shortname = section_shortname.to_lowercase();
        config_load_includes.push(format!(
            r#"#include "hphp/runtime/base/configs/{}-loader.h""#,
            lower_section_shortname
        ));
        config_load_calls.push(format!(
            "  Cfg::{}Loader::Load(ini, config);",
            section_shortname
        ));
        if section
            .configs
            .iter()
            .any(|c| c.features.compiler_option.is_some())
        {
            config_load_compiler_calls.push(format!(
                "  Cfg::{}Loader::LoadForCompiler(ini, config);",
                section_shortname
            ));
        }

        if section
            .configs
            .iter()
            .any(|c| c.features.repo_options_flag.is_some())
        {
            repo_options_flags_calls.push(format!(
                "  Cfg::{}Loader::GetRepoOptionsFlags(flags, ini, config);",
                section_shortname,
            ));
            repo_options_flags_from_config_calls.push(format!(
                "  Cfg::{}Loader::GetRepoOptionsFlagsFromConfig(flags, config, default_flags);",
                section_shortname,
            ));
            repo_options_flags_for_systemlib_calls.push(format!(
                "  Cfg::{}Loader::GetRepoOptionsFlagsForSystemlib(flags);",
                section_shortname,
            ));
        }
    }

    let configs_load_content = format!(
        r#"#include "hphp/runtime/base/configs/configs.h"

{}

namespace HPHP {{

struct IniSettingMap;
struct Hdf;
struct RepoOptionsFlags;

namespace Cfg {{

void Load(const IniSettingMap& ini, const Hdf& config) {{
{}
}}

void LoadForCompiler(const IniSettingMap& ini, const Hdf& config) {{
{}
}}

void GetRepoOptionsFlags(RepoOptionsFlags& flags, const IniSettingMap& ini, const Hdf& config) {{
{}
}}

void GetRepoOptionsFlagsFromConfig(RepoOptionsFlags& flags, const Hdf& config, const RepoOptionsFlags& default_flags) {{
{}
}}

void GetRepoOptionsFlagsForSystemlib(RepoOptionsFlags& flags) {{
{}
}}

}} // namespace Cfg

}} // namespace HPHP
"#,
        config_load_includes.join("\n"),
        config_load_calls.join("\n"),
        config_load_compiler_calls.join("\n"),
        repo_options_flags_calls.join("\n"),
        repo_options_flags_from_config_calls.join("\n"),
        repo_options_flags_for_systemlib_calls.join("\n"),
    );
    let mut configs_load_file = output_dir.clone();
    configs_load_file.push("configs-generated.cpp");
    fs::write(configs_load_file, configs_load_content).unwrap();
}

fn main() -> ExitCode {
    let args = Arguments::parse();

    println!("OUT {:?} {:?}", args.output_dir, args.input);

    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");

    let res = parse_option_doc::<VerboseError<&str>>(&contents);
    match res {
        Ok((_, sections)) => {
            match args.output_type {
                OutputType::Defs => generate_defs(sections, args.output_dir),
                OutputType::Loader => generate_loader(sections, args.output_dir),
            }
            ExitCode::from(0)
        }
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            println!(
                "error parsing header: {}",
                convert_error(contents.as_str(), e)
            );
            ExitCode::from(1)
        }
        Err(Err::Incomplete(_)) => ExitCode::from(2),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_string() {
        let res = parse_string::<VerboseError<&str>>(r#""foo""#);
        assert_eq!(res, Ok(("", r#""foo""#)));
        let res = parse_string::<VerboseError<&str>>(r#""f\\\"o\"o""#);
        assert_eq!(res, Ok(("", r#""f\\\"o\"o""#)));
    }

    #[test]
    fn test_parse_num_expr() {
        let res = parse_num_expr::<VerboseError<&str>>(r#"1 + 2 * 3 - 4 / 5 << 6"#);
        assert_eq!(res, Ok(("", r#"1 + 2 * 3 - 4 / 5 << 6"#)));
        let res = parse_num_expr::<VerboseError<&str>>(r#"( 1 + 2 )"#);
        assert_eq!(res, Ok(("", r#"( 1 + 2 )"#)));
        let res = parse_num_expr::<VerboseError<&str>>(r#"((25 << 10) + 1)"#);
        assert_eq!(res, Ok(("", r#"((25 << 10) + 1)"#)));
        let res = parse_num_expr::<VerboseError<&str>>(r#"(25 << 10) + 1"#);
        assert_eq!(res, Ok(("", r#"(25 << 10) + 1"#)));
        let res = parse_num_expr::<VerboseError<&str>>(r#"15 - (25 << 10) + 1"#);
        assert_eq!(res, Ok(("", r#"15 - (25 << 10) + 1"#)));
    }
}

use anyhow::{bail, Result};
use cfg_if::cfg_if;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn get_code_template() -> String {
    r#"
      <html>
        <head>
            <title> {lang} code </title>
        </head>
        <body>
            {code}
        </body>
    </html>
    "#
    .to_string()
}

pub fn syntax_highlight_code(code: String, lang: String) -> Result<String> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme = &ts.themes["Solarized (dark)"];

    let sr = match ss.find_syntax_by_extension(lang.as_str()) {
        Some(code) => code,
        None => bail!("couldn't find syntax with extension: {}", lang),
    };

    let syntax_code = match highlighted_html_for_string(code.as_str(), &ss, sr, theme) {
        Ok(value) => value,
        Err(err) => bail!("couldn't syntax higlight code: {}", err),
    };

    Ok(get_code_template()
        .replace("{code}", syntax_code.as_str())
        .replace("{lang}", lang.as_str()))
}

pub fn get_web_template() -> String {
    r#"
<form method="post" action="/">
           <textarea name="content" rows="30" maxlength="393216" \="" placeholder="Paste your code here..." cols="80"></textarea>
           <br>
           <label for="language">File Extension:</label>
           <select name="language">
                 <option value="Appfile">
                    Appfile
                 </option>
                 <option value="Berksfile">
                    Berksfile
                 </option>
                 <option value="Brewfile">
                    Brewfile
                 </option>
                 <option value="C">
                    C
                 </option>
                 <option value="Cheffile">
                    Cheffile
                 </option>
                 <option value="DOT">
                    DOT
                 </option>
                 <option value="Deliverfile">
                    Deliverfile
                 </option>
                 <option value="Emakefile">
                    Emakefile
                 </option>
                 <option value="Fastfile">
                    Fastfile
                 </option>
                 <option value="GNUmakefile">
                    GNUmakefile
                 </option>
                 <option value="Gemfile">
                    Gemfile
                 </option>
                 <option value="Guardfile">
                    Guardfile
                 </option>
                 <option value="M">
                    M
                 </option>
                 <option value="Makefile">
                    Makefile
                 </option>
                 <option value="OCamlMakefile">
                    OCamlMakefile
                 </option>
                 <option value="PL">
                    PL
                 </option>
                    R
                 </option>
                 <option value="Rakefile">
                    Rakefile
                 </option>
                 <option value="Rantfile">
                    Rantfile
                 </option>
                 <option value="Rprofile">
                    Rprofile
                 </option>
                 <option value="S">
                    S
                 </option>
                 <option value="SConscript">
                    SConscript
                 </option>
                 <option value="SConstruct">
                    SConstruct
                 </option>
                 <option value="Scanfile">
                    Scanfile
                 </option>
                 <option value="Sconstruct">
                    Sconstruct
                 </option>
                 <option value="Snakefile">
                    Snakefile
                 </option>
                 <option value="Snapfile">
                    Snapfile
                 </option>
                 <option value="Thorfile">
                    Thorfile
                 </option>
                 <option value="Vagrantfile">
                    Vagrantfile
                 </option>
                 <option value="adp">
                    adp
                 </option>
                 <option value="applescript">
                    applescript
                 </option>
                 <option value="as">
                    as
                 </option>
                 <option value="asa">
                    asa
                 </option>
                 <option value="asp">
                    asp
                 </option>
                 <option value="babel">
                    babel
                 </option>
                 <option value="bash">
                    bash
                 </option>
                 <option value="bat">
                    bat
                 </option>
                 <option value="bib">
                    bib
                 </option>
                 <option value="bsh">
                    bsh
                 </option>
                 <option value="build">
                    build
                 </option>
                 <option value="builder">
                    builder
                 </option>
                 <option value="c">
                    c
                 </option>
                 <option value="c++">
                    c++
                 </option>
                 <option value="capfile">
                    capfile
                 </option>
                 <option value="cc">
                    cc
                 </option>
                 <option value="cgi">
                    cgi
                 </option>
                 <option value="cl">
                    cl
                 </option>
                 <option value="clj">
                    clj
                 </option>
                 <option value="cls">
                    cls
                 </option>
                 <option value="cmd">
                    cmd
                 </option>
                 <option value="config.ru">
                    config.ru
                 </option>
                 <option value="cp">
                    cp
                 </option>
                 <option value="cpp">
                    cpp
                 </option>
                 <option value="cpy">
                    cpy
                 </option>
                 <option value="cs">
                    cs
                 </option>
                 <option value="css">
                    css
                 </option>
                 <option value="css.erb">
                    css.erb
                 </option>
                 <option value="css.liquid">
                    css.liquid
                 </option><option value="csx">
                    csx
                 </option><option value="cxx">
                    cxx
                 </option><option value="d">
                    d
                 </option><option value="ddl">
                    ddl
                 </option><option value="di">
                    di
                 </option><option value="diff">
                    diff
                 </option><option value="dml">
                    dml
                 </option><option value="dot">
                    dot
                 </option><option value="dpr">
                    dpr
                 </option><option value="dtml">
                    dtml
                 </option><option value="el">
                    el
                 </option><option value="emakefile">
                    emakefile
                 </option><option value="erb">
                    erb
                 </option><option value="erbsql">
                    erbsql
                 </option><option value="erl">
                    erl
                 </option><option value="es6">
                    es6
                 </option><option value="fasl">
                    fasl
                 </option><option value="fcgi">
                    fcgi
                 </option><option value="gemspec">
                    gemspec
                 </option><option value="go">
                    go
                 </option><option value="gradle">
                    gradle
                 </option><option value="groovy">
                    groovy
                 </option><option value="gvy">
                    gvy
                 </option><option value="gyp">
                    gyp
                 </option><option value="gypi">
                    gypi
                 </option><option value="h">
                    h
                 </option><option value="h++">
                    h++
                 </option><option value="haml">
                    haml
                 </option><option value="hh">
                    hh
                 </option><option value="hpp">
                    hpp
                 </option><option value="hrl">
                    hrl
                 </option><option value="hs">
                    hs
                 </option><option value="htm">
                    htm
                 </option><option value="html">
                    html
                 </option><option value="html.erb">
                    html.erb
                 </option><option value="hxx">
                    hxx
                 </option><option value="inc">
                    inc
                 </option><option value="inl">
                    inl
                 </option><option value="ipp">
                    ipp
                 </option><option value="irbrc">
                    irbrc
                 </option><option value="java">
                    java
                 </option><option value="jbuilder">
                    jbuilder
                 </option><option value="js">
                    js
                 </option><option value="js.erb">
                    js.erb
                 </option><option value="json">
                    json
                 </option><option value="jsp">
                    jsp
                 </option><option value="jsx">
                    jsx
                 </option><option value="l">
                    l
                 </option><option value="lhs">
                    lhs
                 </option><option value="lisp">
                    lisp
                 </option><option value="lsp">
                    lsp
                 </option><option value="ltx">
                    ltx
                 </option><option value="lua">
                    lua
                 </option><option value="m">
                    m
                 </option><option value="mak">
                    mak
                 </option><option value="make">
                    make
                 </option><option value="makefile">
                    makefile
                 </option><option value="markdn">
                    markdn
                 </option><option value="markdown">
                    markdown
                 </option><option value="matlab">
                    matlab
                 </option><option value="md">
                    md
                 </option><option value="mdown">
                    mdown
                 </option><option value="mk">
                    mk
                 </option><option value="ml">
                    ml
                 </option><option value="mli">
                    mli
                 </option><option value="mll">
                    mll
                 </option><option value="mly">
                    mly
                 </option><option value="mm">
                    mm
                 </option><option value="mud">
                    mud
                 </option><option value="opml">
                    opml
                 </option><option value="p">
                    p
                 </option><option value="pas">
                    pas
                 </option><option value="patch">
                    patch
                 </option><option value="php">
                    php
                 </option><option value="php3">
                    php3
                 </option><option value="php4">
                    php4
                 </option><option value="php5">
                    php5
                 </option><option value="php7">
                    php7
                 </option><option value="phps">
                    phps
                 </option><option value="phpt">
                    phpt
                 </option><option value="phtml">
                    phtml
                 </option><option value="pl">
                    pl
                 </option><option value="pm">
                    pm
                 </option><option value="pod">
                    pod
                 </option><option value="podspec">
                    podspec
                 </option><option value="prawn">
                    prawn
                 </option><option value="properties">
                    properties
                 </option><option value="py">
                    py
                 </option><option value="py3">
                    py3
                 </option><option value="pyi">
                    pyi
                 </option><option value="pyw">
                    pyw
                 </option><option value="r">
                    r
                 </option><option value="rabl">
                    rabl
                 </option><option value="rails">
                    rails
                 </option><option value="rake">
                    rake
                 </option><option value="rb">
                    rb
                 </option><option value="rbx">
                    rbx
                 </option><option value="rd">
                    rd
                 </option><option value="re">
                    re
                 </option><option value="rest">
                    rest
                 </option><option value="rhtml">
                    rhtml
                 </option><option value="rjs">
                    rjs
                 </option><option value="rpy">
                    rpy
                 </option><option value="rs">
                    rs
                 </option><option value="rss">
                    rss
                 </option><option value="rst">
                    rst
                 </option><option value="ruby.rail">
                    ruby.rail
                 </option><option value="rxml">
                    rxml
                 </option><option value="s">
                    s
                 </option><option value="sass">
                    sass
                 </option><option value="sbt">
                    sbt
                 </option><option value="scala">
                    scala
                 </option><option value="scm">
                    scm
                 </option><option value="sconstruct">
                    sconstruct
                 </option><option value="sh">
                    sh
                 </option><option value="shtml">
                    shtml
                 </option><option value="simplecov">
                    simplecov
                 </option><option value="sql">
                    sql
                 </option><option value="sql.erb">
                    sql.erb
                 </option><option value="ss">
                    ss
                 </option><option value="sty">
                    sty
                 </option><option value="svg">
                    svg
                 </option><option value="swift">
                    swift
                 </option><option value="t">
                    t
                 </option><option value="tcl">
                    tcl
                 </option><option value="tex">
                    tex
                 </option><option value="textile">
                    textile
                 </option><option value="thor">
                    thor
                 </option><option value="tld">
                    tld
                 </option><option value="tmpl">
                    tmpl
                 </option><option value="tpl">
                    tpl
                 </option><option value="ts">
                    ts
                 </option><option value="tsx">
                    tsx
                 </option><option selected="" value="txt">
                    txt
                 </option><option value="wscript">
                    wscript
                 </option><option value="xhtml">
                    xhtml
                 </option><option value="xml">
                    xml
                 </option><option value="xsd">
                    xsd
                 </option><option value="xslt">
                    xslt
                 </option><option value="yaml">
                    yaml
                 </option><option value="yaws">
                    yaws
                 </option><option value="yml">
                    yml
                 </option>
                 <option value="zsh">
                    zsh
                 </option>
           </select>
           <br>
           <input type="submit" value="Paste!">
        </form>

    "#.to_string()
}

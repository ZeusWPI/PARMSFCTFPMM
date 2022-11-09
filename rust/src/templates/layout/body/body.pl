:- module('body', [body_//1]).

:- use_module(library(http/html_write)).

:- html_meta body_(html,?,?).

body_(Content) -->
    html([
        Content
    ]).
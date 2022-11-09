:- module('handlers', []).

:- use_module('controllers/main_controller').

:- multifile http:location/3.
:- dynamic   http:location/3.

:- http_handler(root(.), main_controller:index, [id(main)]).
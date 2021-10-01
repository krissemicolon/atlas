#!/bin/sh

curl 'http://ip-api.com/line/ip-api.com' && printf "\n\n" && printf "\033[0;32mGeolookup API is up\n" || printf "\033[0;31mGeolookup API Failed [https://ip-api.com]\n"
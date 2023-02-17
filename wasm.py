import shutil
import sys
import http

shutil.copyfile(sys.argv[1], "solar_sailors.wasm")
from http.server import test, SimpleHTTPRequestHandler

test(SimpleHTTPRequestHandler)
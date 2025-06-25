#!/usr/bin/env python3
"""
Simple HTTP server to serve the visualization files.
This helps avoid CORS issues when loading the JSON file.

Usage:
    python3 serve.py [port]

Default port is 8000.
"""

import http.server
import socketserver
import sys
import os

def main():
    port = 8000
    if len(sys.argv) > 1:
        try:
            port = int(sys.argv[1])
        except ValueError:
            print(f"Invalid port: {sys.argv[1]}")
            sys.exit(1)
    
    # Change to the directory containing this script
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    handler = http.server.SimpleHTTPRequestHandler
    
    with socketserver.TCPServer(("", port), handler) as httpd:
        print(f"Serving visualization at http://localhost:{port}")
        print(f"Open http://localhost:{port}/index.html in your browser")
        print("Press Ctrl+C to stop the server")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped")

if __name__ == "__main__":
    main()
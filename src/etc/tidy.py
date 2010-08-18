#!/usr/bin/python

import sys, fileinput, subprocess

err=0
cols=78

try:
    result=subprocess.check_output([ "git", "config", "core.autocrlf" ])
    autocrlf=result.strip() == b"true"
except CalledProcessError:
    autocrlf=False

def report_err(s):
    global err
    print("%s:%d: %s" % (fileinput.filename(), fileinput.filelineno(), s))
    err=1

for line in fileinput.input(openhook=fileinput.hook_encoded("utf-8")):
    if line.find('\t') != -1 and fileinput.filename().find("Makefile") == -1:
        report_err("tab character")

    if not autocrlf and line.find('\r') != -1:
        report_err("CR character")

    line_len = len(line)-2 if autocrlf else len(line)-1
    if line_len > cols:
        report_err("line longer than %d chars" % cols)


sys.exit(err)


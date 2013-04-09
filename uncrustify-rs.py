#!/usr/bin/env python

from __future__ import print_function

import sys, subprocess, re, os, argparse, tempfile

# The standard cat utility with full path
cat = '/bin/cat'

# The name of the uncrustify binary with full path
uncrustify_binary = '/Users/muethisn/Downloads/UniversalIndentGUI_macx/indenters/uncrustify.wrapped'

# The default uncrustify configuration file
uncrustify_config = 'uncrustify.cfg'

# regexp for files that will have their trailing whitespace stripped
cleanup_files = re.compile(r'.*(Makefile.am|configure.ac|dune.module|README|README.SVN|COPYING|INSTALL|TODO|\.cmake|CMakeLists.txt|\.pc\.in)$')

# regexp for files that will be reindented using uncrustify
reindent_files = re.compile(r'.*\.(cpp|hpp|cc|hh|c|h)$')

# regexp for detecting lines that contain editor hints. If editor hint cleanup is turned on, all lines matching this
# expression will be removed
remove_hints = re.compile(r'^//((\s*vi:.+:)|(\s*-\*-.+-\*-))\s*$')

# macro call in albertaextra.hh that uncrustify cannot handle correctly, it stumbles over the if statement without braces around
# the body. The first of these two lines will be used to identify lines containing the problematic expression (via a substring match),
# and those lines will be simply replaced by the second line, which adds braces around the if body.
alberta_error_line = 'FOR_ALL_DOFS(drv->fe_space->admin, if(vec[dof] > maxindex) maxindex = vec[dof] );'
alberta_fixed_line = '  FOR_ALL_DOFS(drv->fe_space->admin, if(vec[dof] > maxindex) { maxindex = vec[dof] } );\n'



# ********************************************************************************
# Set up argument handling
# ********************************************************************************

parser = argparse.ArgumentParser(description='''
Clean up source files, designed to be run as a reposurgeon filter.

Reads the file contents from stdin and writes the processed contents to stdout.
''')

parser.add_argument(
    '-c','--config',
    help = 'The uncrustify configuration file. If this argument is omitted, the file uncrustify.cfg in the current directory is used',
    default = uncrustify_config
)

parser.add_argument(
    '-d','--dump-error-events',
    help = 'Log all events for which an error occured to ERRORFILE. The format is one event per line starting with the event id, followed by the list of blob paths.',
    type = argparse.FileType('a'),
    dest = 'errors',
    metavar = 'ERRORFILE'
)

parser.add_argument(
    '-i','--dump-ignore-events',
    help = 'Log all events that were ignored. The format is one event per line starting with the event id, followed by the list of blob paths.',
    type = argparse.FileType('a'),
    dest = 'ignored',
    metavar = 'IGNOREFILE'
)

parser.add_argument(
    '-s','--skip-events',
    help = 'Specify a file with a list of events to skip and leave unchanged. The format of the file is one event per line, and each line has to start with the event id, followed by a newline or whitespace. The remainder of the line is ignored. Lines starting with "#" are silently ignored.',
    type = argparse.FileType('r'),
    dest = 'skip',
    metavar = 'SKIPFILE'
)

parser.add_argument(
    '-e','--editor-hints',
    help = 'Remove existing emacs and vi editor hints and add standard hints to all files.',
    action = 'store_true',
    dest = 'hints'
)

parser.add_argument(
    '--fix-alberta-macro',
    help = 'Fix offending call to ALBERTA macro that trips up uncrustify.',
    action = 'store_true',
    dest = 'alberta'
)

parser.add_argument(
    '-m','--mark-as-broken',
    help = 'Mark reverted files with a commentary.',
    action = 'store_true'
)

parser.add_argument(
    '-r','--revert-broken',
    help = 'Revert files that caused a problem during the conversion to their originals. Note that this comes with a performance penalty.',
    action = 'store_true'
)

parser.add_argument(
    'event_id',
    help = 'The reposurgeon event id of the blob.',
    type = int
)

parser.add_argument(
    'blob_path',
    help = 'The list of file paths pointing to the blob as supplied by reposurgeon.',
    nargs='+',
)

args = parser.parse_args()

# get name of of uncrustify config file
uncrustify_config = args.config

# set up list of events to skip
if args.skip:
    skip = set(int(line.split()[0]) for line in args.skip if not line.startswith('#'))
    args.skip.close()
else:
    skip = set()


# generator that handles the Alberta problem
def fix_alberta(input):
    for line in input:
        yield alberta_fixed_line if alberta_error_line in line else line

# generator for canonicalizing editor hints
def fix_editor_hints(input):
    yield '// -*- tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*-\n'
    yield '// vi: set et ts=4 sw=2 sts=2:\n'
    for line in input:
        if not remove_hints.match(line):
            yield line

# generator for creating a backup of the input stream to copy, which is supposed to be a file-like object.
def duplicate(input,copy):
    for line in input:
        copy.write(line)
        yield line

if args.event_id in skip:
    # Skip file listed in skipfile
    print('Skipping {}: "{}"'.format(args.event_id,'" "'.join(args.blob_path)),file=sys.stderr)
    os.execl(cat,cat)

elif cleanup_files.match(args.blob_path[0]):
    # Strip trailing whitespace
    print('Fixing trailing whitespace {}: "{}"'.format(args.event_id,'" "'.join(args.blob_path)),file=sys.stderr)

    for line in sys.stdin:
        print(line.rstrip())

elif reindent_files.match(args.blob_path[0]):
    # Reindent
    print('Indenting {}: "{}"'.format(args.event_id,'" "'.join(args.blob_path)),file=sys.stderr)

    # Try to figure out the programming language from the filename
    language = 'C' if args.blob_path[0].endswith(('.c','.h')) else 'CPP'

    if args.revert_broken:
        orig = tempfile.TemporaryFile()

        if args.mark_as_broken:
            orig.write('// NOTE: The current revision of this file was left untouched when the DUNE source files were reindented!\n')
            orig.write('// NOTE: It contained invalid syntax that could not be processed by uncrustify.\n\n')

        source = duplicate(sys.stdin,orig)
    else:
        source = sys.stdin

    if args.alberta:
        source = fix_alberta(source)

    if args.hints:
        source = fix_editor_hints(source)

    if args.revert_broken:
        processed = tempfile.TemporaryFile()
    else:
        processed = sys.stdout

    uncrustify = subprocess.Popen([uncrustify_binary,'-l',language,'-c',uncrustify_config],stdin=subprocess.PIPE,stdout=processed)

    uncrustify_in = uncrustify.stdin
    for line in source:
        uncrustify_in.write(line)
    uncrustify_in.close()

    retcode = uncrustify.wait()

    if retcode != 0 and args.errors:
        args.errors.write('{} "{}"\n'.format(args.event_id,'" "'.join(args.blob_path)))
        args.errors.close()

    if args.revert_broken:
        if retcode == 0:
            processed.seek(0)
            os.dup2(processed.fileno(),sys.stdin.fileno())
        else:
            orig.seek(0)
            os.dup2(orig.fileno(),sys.stdin.fileno())

        orig.close()
        processed.close()
        os.execl(cat,cat)

else:
    # nothing to do here - let cat do the I/O, it's probably faster than us
    print('Ignoring {}: "{}"'.format(args.event_id,'" "'.join(args.blob_path)),file=sys.stderr)

    if args.ignored:
        args.ignored.write('{} "{}"\n'.format(args.event_id,'" "'.join(args.blob_path)))
        args.ignored.close()

    os.execl(cat,cat)

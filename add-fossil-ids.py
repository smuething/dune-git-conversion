#!/usr/bin/env python

import sys, re, os, tempfile

EVENT_DELIMITER = '------------------------------------------------------------------------------\n'

def event_splitter(s):
    # remove initial delimiter
    s.readline()

    event = []
    try:
        for line in s:
            if line == EVENT_DELIMITER:
                yield event
                event = []
            else:
                event.append(line)
    except StopIteration:
        yield event


with open(sys.argv[1],'r') as input, tempfile.NamedTemporaryFile(delete=False) as output:

    tmp_name = output.name

    for event in event_splitter(input):
        # locate SVN id and generate string with backreference
        for line in event:
            if len(line.strip()) == 0:
                # whoops, no fossil id found, report that in the commit message
                msg = '\n[[No corresponding SVN revision found]]\n'
                break
            if line.startswith('Fossil-ID:'):
                # Jackpot!
                svn_id = line.split()[1].strip()
                msg = '\n[[Imported from SVN: r{}]]\n'.format(svn_id)
                break

        output.write(EVENT_DELIMITER)

        for line in event:
            output.write(line)

        output.write(msg)

# replace original with edited file
os.rename(tmp_name,sys.argv[1])

#!/bin/bash

GRIZZLY=$WELD_HOME/examples/python/grizzly

set -x
$GRIZZLY/scripts/replicate-csv -i $GRIZZLY/data/311-service-requests-raw-pruned.csv\
	-o $GRIZZLY/data/311-service-requests.csv\
	-r 400

ipython notebook --no-browser --port=8888




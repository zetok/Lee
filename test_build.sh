#!/bin/bash

# run to test whether it still builds
#
# requires inotify-tools

# pls no spaces
file_for_build_last="build_no"
[ ! -f $file_for_build_last ] && touch $file_for_build_last



# if file modified
while inotifywait src/main.rs;
do

BUILD_LAST=$(cat $file_for_build_last)
BUILD_NOW=$(echo $(($BUILD_LAST + 1)))

if cargo build ; then
	echo $BUILD_NOW > $file_for_build_last
	echo "Build $BUILD_NOW was successful!"

	# commit if wanted
	if [[ "${@}" == 'commit' ]]; then
		git add *
		git commit -am "Successful build $BUILD_NOW"
	fi
else
echo "Build $BUILD_NOW failed!"
#exit 1
fi

done

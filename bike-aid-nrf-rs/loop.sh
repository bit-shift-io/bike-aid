#!/bin/bash
while :
do
	echo "Press [CTRL+C] to stop.."
	probe-rs erase --chip nRF52840_xxAA --allow-erase-all
done

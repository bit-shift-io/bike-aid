#pragma once

#include "Arduino.h"

class Throttle {

private:
  /* 
  PINS
  ===========================
  */
  const byte SMOOTHING_PIN_IN = A1;
  const byte PIN_LIMIT_IN = A2;
  const byte SIGNAL_PIN_IN = A0;
  const byte SIGNAL_PIN_OUT = 10; // 10 = D10

  /* 
  Deadband / Deadzone
  ===========================
  Adjust throttle range to eliminate deadband/deadzones

  MAP_IN - Normal range of throttle
  MAP_OUT - range to output to controller

  All the ranges below can be determined by watching the serial console and twisting the throttle, they will be slightly wrong if the controller supplies less than 5v USB to throttle.
  Preferably, use a multimeter to measure voltage output from the throttle on your ebike and use the formula like so to calculate the numbers:
  ( Signal Voltage / Supply Voltage ) * 1023

  MAP_IN_MIN - Voltage when the throttle is unpressed
  MAP_IN_MAX - Voltage when the throttle is fully pressed
  MAP_OUT_MIN - Voltage just before the motor starts to activate the wheels
  MAP_OUT_MAX - Voltage just after max speed (or use supply voltage otherwise)

  Then verify the output with a multimeter also to tweak the ranges MAP_OUT_MIN, and MAP_OUT_MAX
  */
  // supply voltage - 4.36v
  const int MAP_IN_MIN = 199; // 0.847v no throttle
  const int MAP_IN_MAX = 840; // 3.58v full throttle
  const int MAP_OUT_MIN = 288; // 1.23v just before motor active
  const int MAP_OUT_MAX = 1023; //620 // 2.6v just after max speed

  /* 
  Speed Limit
  ===========================
  adjusts throttle output speed limit
  */
  #define THROTLE_LIMIT_ENABLE // comment to disable speed limit feature
  // pot input is 0-1023, map this to output range
  const int LIMIT_MAP_OUT_MIN = 100;
  const int LIMIT_MAP_OUT_MAX = 1023;

  /* 
  Smoothing - Jerkiness Mitigation
  ===========================
  how quickly to adjust output, larger values are slower
  smoothing over time
  */
  // pot input is 0-1023, map this to output range
  const int SMOOTH_MAP_OUT_MIN = 1; // never zero to avoid divide by zero
  const int SMOOTH_MAP_OUT_MAX = 2000;
  const int DECREASE_SMOOTH_FACTOR = 100;
  //const int INCREASE_SMOOTH_FACTOR = 4000; // potentiometer now

  // Delay between loops in ms
  const int INTERVAL = 1;
  const int DEBUG_PRINT_INTERVAL = 250;

  // global variables
  float output = 0; // 0-1023, later mapped_output to 0-255
  unsigned long last_interval = 0;
  unsigned long last_debug_print_interval = 0;


public:
  Throttle();
  void    init();
  void    update();
};
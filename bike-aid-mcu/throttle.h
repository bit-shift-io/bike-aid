#pragma once
#include "Arduino.h"

class Throttle {
  public:
    void update();
    void setEnable(bool);
    void set_increase_smoothing_factor(int);
    int get_increase_smoothing_factor();
    
    // singleton stuff + delete the functions
    static Throttle& instance();
    Throttle(const Throttle&) = delete;
    Throttle(Throttle&&) = delete;
    Throttle& operator=(const Throttle&) = delete;
    Throttle& operator=(Throttle&&) = delete;

  private:
    bool enabled = false;
    static Throttle& rInstance;
    Throttle();
    //~Throttle();

    // pins
    const byte PIN_IN = A0;
    const byte PIN_OUT = 10; // 10 = D10

    // Delay between loops in ms
    const int INTERVAL = 1;
    const int DEBUG_PRINT_INTERVAL = 250;

    float output = 0; // 0-1023, later mapped_output to 0-255
    unsigned long last_interval = 0;
    unsigned long last_debug_print_interval = 0;

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
    const int LIMIT_MAP_OUT_MIN = 100;
    const int LIMIT_MAP_OUT_MAX = 1023;

    /* 
    Smoothing - Jerkiness Mitigation
    ===========================
    how quickly to adjust output, larger values are slower
    smoothing over time
    */
    const int SMOOTH_MAP_OUT_MIN = 1; // never zero to avoid divide by zero
    const int SMOOTH_MAP_OUT_MAX = 2000; 
    const int DECREASE_SMOOTH_FACTOR = 100;
    int INCREASE_SMOOTH_FACTOR = 4000;  // stored in flash, not const

};
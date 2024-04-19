#include "throttle.h"


Throttle::Throttle() {
  //analogReference(EXTERNAL);
  pinMode(PIN_IN, INPUT);
  pinMode(PIN_OUT, OUTPUT);
  output = analogRead(PIN_IN); // initial value
}


Throttle& Throttle::instance() {
  static auto &&rInstance = Throttle();
  return rInstance;
}


void Throttle::set_enable(bool enable) {
  Log.print("throttle enable ");Log.println(enable);
  enabled = enable;
}


void Throttle::update() {
  if (!enabled)
    return;

  unsigned long time = millis();

  if (time - last_interval > INTERVAL) {
    last_interval = time;

    // throttle hall sensor input
    int input = analogRead(PIN_IN);
    // delta computed smoothing/error from last setting (output)
    int delta = input - output; // smoothing/error

    // how much to change (+/-) throttle value this itr
    float adjustment = (float) delta / (float) (delta > 0 ? INCREASE_SMOOTH_FACTOR : DECREASE_SMOOTH_FACTOR);


    // speed limit
    int limit_input = 1023;
    // Apply speed limit - allow increase only if below limit
    if (output > map(limit_input, 0, 1023, LIMIT_MAP_OUT_MIN, LIMIT_MAP_OUT_MAX)) {
      adjustment = _min(adjustment, 0); // always allow decrease // _min for esp, min for arduino
    }

    output += adjustment;

    // throttle to output value map - mapping to controller range
    int mapped_output = map(output, MAP_IN_MIN, MAP_IN_MAX, MAP_OUT_MIN, MAP_OUT_MAX);

    // PWM is 0-254 while our values are 0-1023
    analogWrite(PIN_OUT, mapped_output / 4);


    if ((last_debug_print_interval + DEBUG_PRINT_INTERVAL) < time) {
      last_debug_print_interval = time;
      // format for serial plotter
      //Log.print(",Th_In:");Log.print(input);
      //Log.print(",Th_Out:");Log.print(output);
      Log.print(",Th_Map:");Log.print(mapped_output);
      //Log.print(",Th_Adj:");Log.print(adjustment);
      Log.println();
    }

  }
}


void Throttle::set_increase_smoothing_factor(int value) {
  INCREASE_SMOOTH_FACTOR = value;
}

int Throttle::get_increase_smoothing_factor() {
  return INCREASE_SMOOTH_FACTOR;
}
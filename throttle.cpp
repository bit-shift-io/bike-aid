#include "Arduino.h"
#include "throttle.h"

Throttle::Throttle() {

}


void Throttle::update() {
  unsigned long time = millis();

  if ((last_interval + INTERVAL) < time) {
    last_interval = time;

    // throttle hall sensor input
    int input = analogRead(SIGNAL_PIN_IN);
    // delta computed smoothing/error from last setting (output)
    int delta = input - output; // smoothing/error

    // smoothing potentiometer
    int smooth_input = analogRead(SMOOTHING_PIN_IN);
    int smooth_mapped = map(smooth_input, 0, 1023, SMOOTH_MAP_OUT_MIN, SMOOTH_MAP_OUT_MAX); // INCREASE_SMOOTH_FACTOR

    // how much to change (+/-) throttle value this itr
    float adjustment = (float) delta / (float) (delta > 0 ? smooth_mapped : DECREASE_SMOOTH_FACTOR);

    #ifdef THROTLE_LIMIT_ENABLE
    // limit potentiometer
    int limit_input = analogRead(PIN_LIMIT_IN);
    // Apply speed limit - allow increase only if below limit
    if (output > map(limit_input, 0, 1023, LIMIT_MAP_OUT_MIN, LIMIT_MAP_OUT_MAX)) {
      adjustment = min(adjustment, 0); // always allow decrease
    }
    #endif

    output += adjustment;

    // throttle to output value map - mapping to controller range
    int mapped_output = map(output, MAP_IN_MIN, MAP_IN_MAX, MAP_OUT_MIN, MAP_OUT_MAX);

    // PWM is 0-254 while our values are 0-1023
    analogWrite(SIGNAL_PIN_OUT, mapped_output / 4);

    #ifdef ARDUINO_AVR_NANO
    if ((last_debug_print_interval + DEBUG_PRINT_INTERVAL) < time) {
      last_debug_print_interval = time;
      // format for serial plotter
      //Serial.print(",Th_In:");Serial.print(input);
      //Serial.print(",Th_Out:");Serial.print(output);
      Serial.print(",Th_Map:");Serial.print(mapped_output);
      //Serial.print(",Th_Adj:");Serial.print(adjustment);
      Serial.print(",Lim_In:");Serial.print(limit_input);
      Serial.print(",Smo_In:");Serial.print(smooth_input);
      Serial.print(",G1:");Serial.print(MAP_OUT_MIN); // guide
      Serial.print(",G2:");Serial.print(MAP_OUT_MAX); // guide
      Serial.println();
    }
    #endif

  }
}

void Throttle::init() {
  pinMode(SMOOTHING_PIN_IN, INPUT);
  pinMode(PIN_LIMIT_IN, INPUT);
  pinMode(SIGNAL_PIN_IN, INPUT);
  pinMode(SIGNAL_PIN_OUT, OUTPUT); // output

  output = analogRead(SIGNAL_PIN_IN); // initial value

  // safety feature for disconnected throttle
  // ensure throttle is not in use
  while(analogRead(SIGNAL_PIN_IN) > 400) // todo: use MAP_OUT_MIN
  {
    #ifdef ARDUINO_AVR_NANO
    Serial.println("Error: Throttle wire has no signal!");
    Serial.println(analogRead(SIGNAL_PIN_IN));
    #endif
  }

  // wait for sensors to stabalise
  delay(100);
}
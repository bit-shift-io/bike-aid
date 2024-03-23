/**
 * Based on Akom's smooth throttle using an Arduino
 * https://github.com/akomakom/arduino-throttle-smoother
 * 
 * Smoothing jerky throttle response - adjustable by potentiometer
 * Minimize deadband/deadzones
 * Speed Limit - adjustable by potentiometer
 */

/* 
PINS
===========================
*/

#ifdef ARDUINO_AVR_NANO 
const byte THROTTLE_SIGNAL_PIN_IN = A0;
const byte THROTTLE_SMOOTHING_PIN_IN = A1;
const byte THROTTLE_PIN_LIMIT_IN = A2;
const byte THROTTLE_SIGNAL_PIN_OUT = 10; // 10 = D10
#else // ATTINY
const byte THROTTLE_SIGNAL_PIN_IN = 7;
const byte THROTTLE_SMOOTHING_PIN_IN = 2;
const byte THROTTLE_PIN_LIMIT_IN = 3;
const byte THROTTLE_SIGNAL_PIN_OUT = 6;
#endif



/**
 * All the ranges below can be determined by watching the serial console and twisting the throttle
 * Note that they will be slightly wrong if the controller supplies less than 5v to throttle.
 */


/* 
Deadband 
===========================
fine tune the throttle range to eliminate deadband 
MAP_IN - Normal range of throttle
MAP_OUT - range to output to controller
*/
const int THROTTLE_MAP_IN_MIN = 180; // no throttle
const int THROTTLE_MAP_IN_MAX = 850; // full throttle
const int THROTTLE_MAP_OUT_MIN = 390;
const int THROTTLE_MAP_OUT_MAX = 800;

/* 
Speed Limit
===========================
adjusts throttle output speed limit
*/
#define THROTLE_LIMIT_ENABLE // comment to disable speed limit feature
// pot input is 0-1023, map this to output range
const int THROTTLE_LIMIT_MAP_OUT_MIN = 100;
const int THROTTLE_LIMIT_MAP_OUT_MAX = 1023;

/* 
Smoothing - Jerkiness Mitigation
===========================
how quickly to adjust output, larger values are slower
smoothing over time
*/
// pot input is 0-1023, map this to output range
const int THROTTLE_SMOOTH_MAP_OUT_MIN = 0;
const int THROTTLE_SMOOTH_MAP_OUT_MAX = 5000;
//const int THROTTLE_INCREASE_SMOOTH_FACTOR = 4000; // potentiometer now
const int THROTTLE_DECREASE_SMOOTH_FACTOR = 100;

// Delay between loops in ms
const int THROTTLE_INTERVAL = 1;
const int DEBUG_PRINT_INTERVAL = 500;

// global variables
float throttle_output = 0; // 0-1024, later throttle_mapped_output to 0-255
unsigned long last_throttle_interval = 0;
unsigned long last_debug_print_interval = 0;


void setup() {
  analogReference(EXTERNAL);

  #ifdef ARDUINO_AVR_NANO
  Serial.begin(9600);
  #endif

  pinMode(THROTTLE_SIGNAL_PIN_IN, INPUT);
  pinMode(THROTTLE_PIN_LIMIT_IN, INPUT);
  pinMode(THROTTLE_SIGNAL_PIN_OUT, OUTPUT);
  throttle_output = analogRead(THROTTLE_SIGNAL_PIN_IN); // initial value
  // safety feature for disconnected throttle
  // ensure throttle is not in use
  while(analogRead(THROTTLE_SIGNAL_PIN_IN) >= 250)
  {
    #ifdef ARDUINO_AVR_NANO
    Serial.println("Error: Throttle wire has no signal!");
    Serial.println(analogRead(THROTTLE_SIGNAL_PIN_IN));
    #endif
  }

  // wait for sensors to stabalise
  delay(1000);
}

void loop() {
  throttle();
}


void throttle() {
  if ((last_throttle_interval + THROTTLE_INTERVAL) < millis()) {
    last_throttle_interval = millis();

    // throttle hall sensor input
    int throttle_input = analogRead(THROTTLE_SIGNAL_PIN_IN);
    // delta computed smoothing/error from last setting (throttle_output)
    int throttle_delta = throttle_input - throttle_output; // smoothing/error

    // smoothing potentiometer
    int throttle_smooth_input = analogRead(THROTTLE_SMOOTHING_PIN_IN);
    int throttle_smooth_mapped = map(throttle_smooth_input, 0, 1023, THROTTLE_SMOOTH_MAP_OUT_MIN, THROTTLE_SMOOTH_MAP_OUT_MAX); // THROTTLE_INCREASE_SMOOTH_FACTOR

    // how much to change (+/-) throttle value this itr
    float throttle_adjustment = (float) throttle_delta / (float) (throttle_delta > 0 ? throttle_smooth_mapped : THROTTLE_DECREASE_SMOOTH_FACTOR);

    #ifdef THROTLE_LIMIT_ENABLE
    // limit potentiometer
    int throttle_limit_input = analogRead(THROTTLE_PIN_LIMIT_IN);
    // Apply speed limit - allow increase only if below limit
    if (throttle_output > map(throttle_limit_input, 0, 1023, THROTTLE_LIMIT_MAP_OUT_MIN, THROTTLE_LIMIT_MAP_OUT_MAX)) {
      throttle_adjustment = min(throttle_adjustment, 0); // always allow decrease
    }
    #endif

    throttle_output += throttle_adjustment;

    // throttle to output value map - mapping to controller range
    int throttle_mapped_output = map(
      throttle_output,
      THROTTLE_MAP_IN_MIN,
      THROTTLE_MAP_IN_MAX,
      THROTTLE_MAP_OUT_MIN,
      THROTTLE_MAP_OUT_MAX
    );

    analogWrite(
      THROTTLE_SIGNAL_PIN_OUT,
      throttle_mapped_output / 4 // PWM is 0-254 while our values are 0-1023
    );

    #ifdef ARDUINO_AVR_NANO
    if ((last_debug_print_interval + DEBUG_PRINT_INTERVAL) < millis()) {
      last_debug_print_interval = millis();

      // format for serial plotter
      Serial.print(",Throttle In:");
      Serial.print(throttle_input);

      Serial.print(",Throttle Out:");
      Serial.print(throttle_output);

      Serial.print(",Throttle Fin:");
      Serial.print(throttle_mapped_output);

      Serial.println();

/*
      Serial.print("throttle_input\t"); Serial.println(throttle_input);

      Serial.print("throttle_output\t"); Serial.println(throttle_output);
      Serial.print("throttle_mapped_output\t"); Serial.println(throttle_mapped_output);

      Serial.print("throttle_adjustment\t"); Serial.println(throttle_adjustment);

      Serial.print("throttle_limit_input\t"); Serial.println(throttle_limit_input);
      Serial.print("throttle_limit_output\t"); Serial.println(map(throttle_limit_input, 0, 1023, THROTTLE_LIMIT_MAP_OUT_MIN, THROTTLE_LIMIT_MAP_OUT_MAX));

      Serial.print("throttle_smooth_input\t"); Serial.println(throttle_smooth_input);
      Serial.print("throttle_smooth_mapped\t"); Serial.println(throttle_smooth_mapped);

      Serial.print("Throttle In -> Out -> Map\tTh Ad\tLimit In -> Map\tSmooth In -> Map");Serial.print("\n");

      Serial.print("     ");Serial.print(throttle_input);Serial.print(" -> ");
      Serial.print(throttle_output);Serial.print(" -> ");

      Serial.print(throttle_mapped_output);Serial.print("\t");

      Serial.print(throttle_adjustment);Serial.print("\t");

      Serial.print(throttle_limit_input);Serial.print("->");
      Serial.print(map(throttle_limit_input, 0, 1023, THROTTLE_LIMIT_MAP_OUT_MIN, THROTTLE_LIMIT_MAP_OUT_MAX));Serial.print("\t");

      Serial.print(throttle_smooth_input);Serial.print(" -> ");
      Serial.print(throttle_smooth_mapped);

      Serial.print("\n\n");
      */
    }
    #endif

  }
}

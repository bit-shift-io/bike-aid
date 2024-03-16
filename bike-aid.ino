/**
 * Based on Akom's smooth throttle using an Arduino
 * https://github.com/akomakom/arduino-throttle-smoother
 * 
 * Smoothing jerky throttle response - adjustable by potentiometer
 * Minimize deadband/deadzones
 * Speed Limit - adjustable by potentiometer
 */

// print output messages - comment to disable
#define DEBUG_ENABLE

/* 
PINS
===========================
*/
const byte THROTTLE_SIGNAL_PIN_IN = A1;
const byte THROTTLE_SIGNAL_PIN_OUT = 10; // D7
const byte THROTTLE_PIN_LIMIT = A3;
const byte THROTTLE_PIN_SMOOTHING = A1;


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
const int THROTTLE_MAP_IN_MIN = 180;
const int THROTTLE_MAP_IN_MAX = 850;
const int THROTTLE_MAP_OUT_MIN = 390;
const int THROTTLE_MAP_OUT_MAX = 800;

/* 
Speed Limit
===========================
*/
#define THROTLE_LIMIT_ENABLE // comment to disable speed limit feature
/* fine tune the speed limit potentiometer range */
// input from potentiometer - possible values on analogRead, adjust to your pot's range
// (min from pot to max from pot).   Defaults are appropriate for a 1K ohm pot.
const int THROTTLE_LIMIT_MAP_IN_MIN = 0;
const int THROTTLE_LIMIT_MAP_IN_MAX = 1023;
// this adjusts throttle output speed limit
// value is applied to the throttle input range (THROTTLE_MAP_IN_*)
// Adjust to "about as slow as is practical" to "max speed" - change the added number to your needs
// you can also subtract from max to disallow full speed (hardcoded speed limit)
// like this:  #define THROTTLE_LIMIT_MAP_OUT_MAX THROTTLE_MAP_IN_MAX - 300
const int THROTTLE_LIMIT_MAP_OUT_MIN = THROTTLE_MAP_IN_MIN + 100;
const int THROTTLE_LIMIT_MAP_OUT_MAX = THROTTLE_MAP_IN_MAX;

/* 
Smoothing - Jerkiness Mitigation
===========================
how quickly to adjust output, larger values are slower
smoothing over time
*/
const int THROTTLE_INCREASE_SMOOTH_FACTOR = 10000;
const int THROTTLE_DECREASE_SMOOTH_FACTOR = 100;

// Delay between loops
const int THROTTLE_INTERVAL = 1; // ms
unsigned long last_throttle_interval = 0;
const int DEBUG_PRINT_INTERVAL = 100;
unsigned long last_debug_print_interval = 0;

// operational global variables
int throttle_input = 0;         //input value from 3-wire throttle 
int throttle_limit_input = 0;   //input value from potentiometer 
float throttle_output = 0;      // 0-1024, later throttle_mapped_output to 0-255
float throttle_adjustment = 0;  // 
int throttle_mapped_output = 0; // throttle_output after mapping to controller range


void setup() {
    Serial.begin(9600);
    pinMode(THROTTLE_SIGNAL_PIN_IN, INPUT);
    pinMode(THROTTLE_PIN_LIMIT, INPUT);
    pinMode(THROTTLE_SIGNAL_PIN_OUT, OUTPUT);
    throttle_output = analogRead(THROTTLE_SIGNAL_PIN_IN); // initial value
}

void loop() {
  throttle();
  debug();
}

void debug() {
#ifdef DEBUG_ENABLE
  if ((last_debug_print_interval + DEBUG_PRINT_INTERVAL) < millis()) {
    last_debug_print_interval = millis();

    Serial.print("Input: ");
    Serial.print(throttle_input);
    Serial.print(" Output: ");
    Serial.print(throttle_output);
    Serial.print(" Mapped: ");
    Serial.print(throttle_mapped_output);
    Serial.print(" +/-: ");
    Serial.print(throttle_adjustment);
    Serial.print(" Lim: ");
    Serial.print(throttle_limit_input);
    Serial.print(" Lim Map: ");
    Serial.print(map(throttle_limit_input, THROTTLE_LIMIT_MAP_IN_MIN, THROTTLE_LIMIT_MAP_IN_MAX, THROTTLE_LIMIT_MAP_OUT_MIN, THROTTLE_LIMIT_MAP_OUT_MAX));
    Serial.println("");
  }
#endif
}

void throttle() {
  if ((last_throttle_interval + THROTTLE_INTERVAL) < millis()) {
    last_throttle_interval = millis();

    throttle_input = analogRead(THROTTLE_SIGNAL_PIN_IN);
    // delta computed error from last setting (throttle_output)
    int throttle_delta = throttle_input - throttle_output; // error
    throttle_adjustment = (float) throttle_delta / (float) (throttle_delta > 0 ? THROTTLE_INCREASE_SMOOTH_FACTOR : THROTTLE_DECREASE_SMOOTH_FACTOR);

#ifdef THROTLE_LIMIT_ENABLE
    throttle_limit_input = analogRead(THROTTLE_PIN_LIMIT);
    // Apply speed limit - allow increase only if below limit
    if (throttle_output > map(throttle_limit_input, THROTTLE_LIMIT_MAP_IN_MIN, THROTTLE_LIMIT_MAP_IN_MAX, THROTTLE_LIMIT_MAP_OUT_MIN, THROTTLE_LIMIT_MAP_OUT_MAX)) {
        throttle_adjustment = min(throttle_adjustment, 0); // always allow decrease
    }
#endif
    throttle_output += throttle_adjustment;

    // throttle to output value map
    throttle_mapped_output = map(
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

  }
}

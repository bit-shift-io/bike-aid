package com.bitshift.bike_aid;

import android.bluetooth.BluetoothAdapter;
import android.content.Intent;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.widget.Button;
import android.widget.GridLayout;
import android.widget.ScrollView;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.content.ContextCompat;

public class MainActivity extends AppCompatActivity {
    // ==== notes ====
    /*
    main activity and gui stuff here
     */

    // ==== variables ====
    private static final Logger log = Logger.getInstance();
    private static final BLE ble = BLE.getInstance();
    private static final Signals signals = Signals.getInstance();
    int REQUEST_ENABLE_BT = 1;

    // ==== functions ====

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // gui
        setContentView(R.layout.activity_main);
        initGui();

        // check permissions & init
        new Permissions(this);
        ble.init(getApplicationContext());
        ble.setOnRequestEnableBLEListener(this::onRequestEnableBLE);
    }


    private void onRequestEnableBLE() {
        // check ble is still enabled
        Intent enableBtIntent = new Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE);
        startActivityForResult(enableBtIntent, REQUEST_ENABLE_BT);
    }


    @Override
    protected void onDestroy() {
        super.onDestroy();
        ble.close();
    }

    @Override
    protected void onStop() {
        super.onStop();
        // called before app losses focus, or we can use onpause for dialogs
        ble.stopScan();
    }


    @Override
    protected void onResume() {
        // this is called each time the app is brought to focus and on first start
        super.onResume();

        // connect or reconnect - this will also scan if it needs
        ble.connect();
    }


    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == REQUEST_ENABLE_BT) {
            ble.connect();
        }
    }


    // connect gui to signals
    private void initGui() {
        // reset log button
        Button logReset = findViewById(R.id.log_reset);
        logReset.setOnClickListener(v -> log.reset());

        // send uart button
        Button sendBtn = findViewById(R.id.send_button);
        sendBtn.setOnClickListener(v -> {
            TextView tv = findViewById(R.id.send_text);
            signals.setUART(tv.getText().toString());
            tv.setText("");
        });

        // power button
        Button powerBtn = findViewById(R.id.power_button);
        powerBtn.setOnClickListener(v -> signals.togglePower());

        // alarm button
        Button alarmBtn = findViewById(R.id.alarm_button);
        alarmBtn.setOnClickListener(v -> signals.toggleAlarm());

        // callaback for log updates
        ScrollView logScroll = findViewById(R.id.log_scroll);
        TextView logText = findViewById(R.id.log_text);
        log.setOnEventListener(result -> {
            logText.setText(result);
            new Handler().postDelayed(() -> logScroll.fullScroll(View.FOCUS_DOWN), 10);
        });

        // callback for signals
        signals.setOnEventListener(new Signals.OnEventListener() {
            public void onCruiseLevel(int level) {
                // Loop through the views and set their visibility based on the level
                GridLayout gridLayout = findViewById(R.id.cruise_level);
                for (int i = 0; i < gridLayout.getChildCount(); i++) {
                    View view = gridLayout.getChildAt(i);
                    if (i < (5 - level)) {
                        view.setVisibility(View.INVISIBLE); // Show the view for levels 1 to level
                    } else {
                        view.setVisibility(View.VISIBLE); // Hide the view for levels above the current level
                    }
                }

                TextView item = findViewById(R.id.cruise);
                if (level > 0) {
                    item.setVisibility(View.VISIBLE);
                } else {
                    item.setVisibility(View.INVISIBLE);
                }
            }
            @Override
            public void onTemperature(String result) {
                TextView item = findViewById(R.id.temperature);
                item.setText(result);
            }

            @Override
            public void onSpeed(String result) {
                TextView item = findViewById(R.id.speed);
                item.setText(result);
            }

            @Override
            public void onClockMinutes(String result) {
                TextView item = findViewById(R.id.clock_minutes);
                item.setText(result);
            }

            @Override
            public void onClockHours(String result) {
                TextView item = findViewById(R.id.clock_hours);
                item.setText(result);
            }

            @Override
            public void onPower(boolean result) {
                Button item = findViewById(R.id.power_button);
                if (result) {
                    item.setText("Power\nON");
                    item.setBackgroundTintList(ContextCompat.getColorStateList(getApplicationContext(), R.color.buttonOn));
                }
                else {
                    item.setText("Power\nOFF");
                    item.setBackgroundTintList(ContextCompat.getColorStateList(getApplicationContext(), R.color.buttonOff));
                }
            }

            @Override
            public void onAlarm(boolean result) {
                Button item = findViewById(R.id.alarm_button);
                if (result) {
                    item.setText("Alarm\nON");
                    item.setBackgroundTintList(ContextCompat.getColorStateList(getApplicationContext(), R.color.buttonOn));
                }
                else {
                    item.setText("Alarm\nOFF");
                    item.setBackgroundTintList(ContextCompat.getColorStateList(getApplicationContext(), R.color.buttonOff));
                }
            }

            @Override
            public void onBatteryLevel(String result) {
                TextView item = findViewById(R.id.battery_level);
                item.setText(result);
            }

            @Override
            public void onBrake(boolean result) {
                TextView item = findViewById(R.id.brake);
                if (result)
                    item.setVisibility(View.VISIBLE);
                else
                    item.setVisibility(View.INVISIBLE);
            }

            @Override
            public void onParkBrake(boolean result) {
                TextView item = findViewById(R.id.park_brake);
                if (result)
                    item.setVisibility(View.VISIBLE);
                else
                    item.setVisibility(View.INVISIBLE);
            }

            @Override
            public void onBatteryPower(String result) {
                TextView item = findViewById(R.id.battery_power);
                item.setText(result);
            }

            @Override
            public void onThrottleLevel(String result) {
                TextView item = findViewById(R.id.throttle_level);
                item.setText(result);
            }
        });
    }
}
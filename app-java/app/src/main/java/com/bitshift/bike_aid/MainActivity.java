package com.bitshift.bike_aid;

import android.bluetooth.BluetoothAdapter;
import android.content.Intent;
import android.content.res.ColorStateList;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.widget.Button;
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

        // check permissions
        new Permissions(this);

        // start ble
        ble.init(getApplicationContext());
        if (!ble.isEnabled()) {
            Intent enableBtIntent = new Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE);
            startActivityForResult(enableBtIntent, REQUEST_ENABLE_BT);
        }
    }


    @Override
    protected void onDestroy() {
        super.onDestroy();
        ble.close();
    }


    @Override
    protected void onResume() {
        super.onResume();
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

        // callaback for log updates
        ScrollView logScroll = findViewById(R.id.log_scroll);
        TextView logText = findViewById(R.id.log_text);
        log.setOnEventListener(new Logger.OnEventListener() {
            @Override
            public void onUpdate(String result) {
                logText.setText(result);
                new Handler().postDelayed(() -> logScroll.fullScroll(View.FOCUS_DOWN), 10);
            }
        });

        // callback for signals
        signals.setOnEventListener(new Signals.OnEventListener() {
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
        });

        // reset log button
        Button logReset = findViewById(R.id.log_reset);
        logReset.setOnClickListener( new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                log.reset();
            }
        });

        // send uart button
        Button sendBtn = findViewById(R.id.send_button);
        sendBtn.setOnClickListener( new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                TextView tv = findViewById(R.id.send_text);
                signals.setUART(tv.getText().toString());
                tv.setText("");
            }
        });

        // power button
        Button powerBtn = findViewById(R.id.power_button);
        powerBtn.setOnClickListener( new View.OnClickListener() {
            @Override
            public void onClick(View v) { signals.togglePower(); }
        });

        // alarm button
        Button alarmBtn = findViewById(R.id.alarm_button);
        alarmBtn.setOnClickListener( new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                signals.toggleAlarm();
            }
        });

    }


}
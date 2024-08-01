package com.bitshift.bike_aid;

import android.content.Context;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.view.Window;
import android.widget.Button;
import android.widget.ScrollView;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {
    // ==== notes ====
    /*
    main activity and gui stuff here
     */

    // ==== variables ====
    private static final Logger log = Logger.getInstance();
    private static final BLE ble = BLE.getInstance();
    private static final GATT gatt = GATT.getInstance();
    ScrollView logScroll;
    TextView logText;


    // ==== functions ====

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // gui
        setContentView(R.layout.activity_main);
        Context context = getApplicationContext();

        // store views
        logScroll = findViewById(R.id.log_scroll);
        logText = findViewById(R.id.log_text);

        // callaback for log updates
        log.setOnEventListener(new Logger.OnEventListener() {
            @Override
            public void onUpdate(String result) {
                logText.setText(result);
                new Handler().postDelayed(() -> logScroll.fullScroll(View.FOCUS_DOWN), 10);
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
                log.info("send: " + tv.getText().toString());
                tv.setText("");
            }
        });

        // power button
        Button powerBtn = findViewById(R.id.power_button);
        powerBtn.setOnClickListener( new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                log.info("TODO: power");
            }
        });

        // alarm button
        Button alarmBtn = findViewById(R.id.alarm_button);
        alarmBtn.setOnClickListener( new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                log.info("TODO: alarm");
            }
        });

        log.info("activity: create");

        // check permissions
        new Permissions(this);

        // start ble
        ble.init(context);
    }


}
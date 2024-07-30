package com.bitshift.bike_aid;

import android.content.Context;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.view.Window;
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
        logScroll = findViewById(R.id.scrollViewLog);
        logText = findViewById(R.id.textViewLog);

        // callaback for log updates
        log.setOnEventListener(new Logger.OnEventListener() {
            @Override
            public void onUpdate(String result) {
                logText.setText(result);
                new Handler().postDelayed(() -> logScroll.fullScroll(View.FOCUS_DOWN), 10);
            }
        });

        log.info("activity: create");

        // check permissions
        new Permissions(this);

        // start ble
        ble.init(context);
    }


}